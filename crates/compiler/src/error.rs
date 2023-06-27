use crate::prelude::Str;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, PartialEq)]
pub enum Error {
    UnexpectedEof,
    UnexpectedToken(Str),
    UnexpectedExpression,
    InvalidEscapeSequence(Str),
    InvalidFloat(Str),
    InvalidToken,
    InvalidUnaryExpression,
    InvalidBinaryExpression,
}
