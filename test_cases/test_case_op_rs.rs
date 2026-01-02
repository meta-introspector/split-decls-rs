// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_hir_typeck/src/op.rs
// Error: expected square brackets
// Problematic line: line 8

use rustc_errors::{Applicability, Diag, struct_span_code_err};
use rustc_infer::traits::ObligationCauseCode;
use rustc_middle::bug;
use rustc_middle::ty::adjustment::{
    Adjust, Adjustment, AllowTwoPhase, AutoBorrow, AutoBorrowMutability,
};
use rustc_middle::ty::print::with_no_trimmed_paths;
