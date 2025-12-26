use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// A trait for parsing Rust source code and extracting structured declarations.
pub trait RustAstParser {
    /// Parses a given string of Rust source code and returns a vector of `Declaration`s.
    fn parse_rust_code(&self, code: &str) -> Vec<Declaration>;
}
