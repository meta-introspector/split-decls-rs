use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// A simple wrapper round a string, used for errors reported from
/// ffi calls.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Error {
    message: String,
}
