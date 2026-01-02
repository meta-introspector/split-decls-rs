// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_builtin_macros/src/deriving/generic/ty.rs
// Error: expected square brackets
// Problematic line: line 11

use rustc_span::{DUMMY_SP, Ident, Span, Symbol, kw};
use thin_vec::ThinVec;

/// A path, e.g., `::std::option::Option::<i32>` (global). Has support
/// for type parameters.
#[derive(Clone)]
pub(crate) struct Path {
