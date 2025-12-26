use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// A piece is a portion of the format string which represents the next part
/// to emit. These are emitted as a stream by the `Parser` class.
#[derive(Clone, Debug, PartialEq)]
pub enum Piece<'input> {
    /// A literal string which should directly be emitted
    Lit(&'input str),
    /// This describes that formatting should process the next argument (as
    /// specified inside) for emission.
    NextArgument(Box<Argument<'input>>),
}
