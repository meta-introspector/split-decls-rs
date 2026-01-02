// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_const_eval/src/check_consts/qualifs.rs
// Error: expected square brackets
// Problematic line: line 19


use super::ConstCx;

pub fn in_any_value_of_ty<'tcx>(
    cx: &ConstCx<'_, 'tcx>,
    ty: Ty<'tcx>,
    tainted_by_errors: Option<ErrorGuaranteed>,
