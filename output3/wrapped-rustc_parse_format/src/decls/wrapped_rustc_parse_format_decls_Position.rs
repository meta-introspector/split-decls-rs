use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Enum describing where an argument for a format can be located.
#[derive(Clone, Debug, PartialEq)]
pub enum Position<'input> {
    /// The argument is implied to be located at an index
    ArgumentImplicitlyIs(usize),
    /// The argument is located at a specific index given in the format,
    ArgumentIs(usize),
    /// The argument has a name.
    ArgumentNamed(&'input str),
}
