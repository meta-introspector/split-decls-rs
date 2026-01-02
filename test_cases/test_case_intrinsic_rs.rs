// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_hir_analysis/src/check/intrinsic.rs
// Error: expected square brackets
// Problematic line: line 14

use crate::check::check_function_signature;
use crate::errors::{UnrecognizedIntrinsicFunction, WrongNumberOfGenericArgumentsToIntrinsic};

fn equate_intrinsic_type<'tcx>(
    tcx: TyCtxt<'tcx>,
    span: Span,
    def_id: LocalDefId,
