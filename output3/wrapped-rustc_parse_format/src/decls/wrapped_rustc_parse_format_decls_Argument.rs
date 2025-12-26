use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Representation of an argument specification.
#[derive(Clone, Debug, PartialEq)]
pub struct Argument<'input> {
    /// Where to find this argument
    pub position: Position<'input>,
    /// The span of the position indicator. Includes any whitespace in implicit
    /// positions (`{  }`).
    pub position_span: Range<usize>,
    /// How to format the argument
    pub format: FormatSpec<'input>,
}
