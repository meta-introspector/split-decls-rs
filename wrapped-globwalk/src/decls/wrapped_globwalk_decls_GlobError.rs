use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Error from parsing globs.
#[derive(Debug)]
pub struct GlobError(ignore::Error);
