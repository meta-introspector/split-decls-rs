// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_macros/src/diagnostics/diagnostic.rs
// Error: expected square brackets
// Problematic line: line 14

use crate::diagnostics::error::{DiagnosticDeriveError, span_err};
use crate::diagnostics::utils::SetOnce;

/// The central struct for constructing the `into_diag` method from an annotated struct.
pub(crate) struct DiagnosticDerive<'a> {
    structure: Structure<'a>,
}
