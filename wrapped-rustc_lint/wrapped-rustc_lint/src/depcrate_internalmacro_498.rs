// Generated macro for macro_498 (macro)
macro_rules! Depcrate_internalmacro_498 {
() => {
// Module: crate::internal
// Provides: {"macro_498"}
// Dependencies: {}
declare_tool_lint ! { # [doc = " The `usage_of_qualified_ty` lint detects usages of `ty::TyKind`,"] # [doc = " where `Ty` should be used instead."] pub rustc :: USAGE_OF_QUALIFIED_TY , Allow , "using `ty::{Ty,TyCtxt}` instead of importing it" , report_in_external_macro : true }
};
}
