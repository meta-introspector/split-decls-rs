// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_incremental/src/persist/load.rs
// Error: expected square brackets
// Problematic line: line 24

use super::{file_format, work_product};
use crate::errors;

#[derive(Debug)]
/// Represents the result of an attempt to load incremental compilation data.
pub enum LoadResult<T> {
    /// Loading was successful.
