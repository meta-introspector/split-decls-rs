// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_data_structures/src/frozen.rs
// Error: expected square brackets
// Problematic line: line 48

//! - `Frozen` doesn't pin it's contents (e.g. one could still do `foo.computed =
//!    Frozen::freeze(new_bar)`).

/// An owned immutable value.
#[derive(Debug, Clone)]
pub struct Frozen<T>(T);

