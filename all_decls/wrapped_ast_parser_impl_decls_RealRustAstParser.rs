use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// A real implementation of `RustAstParser` using `syn` and macro expansion.
#[derive(Debug, Default)]
pub struct RealRustAstParser;
