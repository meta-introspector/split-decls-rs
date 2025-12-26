use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Command-line Argument
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ParsedArg<'s> {
    inner: &'s OsStr,
}
