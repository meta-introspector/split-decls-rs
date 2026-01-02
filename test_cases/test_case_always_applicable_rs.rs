// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_hir_analysis/src/check/always_applicable.rs
// Error: expected square brackets
// Problematic line: line 21

use crate::errors;
use crate::hir::def_id::{DefId, LocalDefId};

/// This function confirms that the `Drop` implementation identified by
/// `drop_impl_did` is not any more specialized than the type it is
/// attached to (Issue #8142).
///
