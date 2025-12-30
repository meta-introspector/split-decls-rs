// Generated macro for impl_2306 (impl)
macro_rules! Depcrate_format_argsimpl_2306 {
() => {
// Module: crate::format_args
// Provides: {"impl_2306"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for FormatArgs < 'tcx > { fn check_expr (& mut self , cx : & LateContext < 'tcx > , expr : & 'tcx Expr < 'tcx >) { if let Some (macro_call) = root_macro_call_first_node (cx , expr) && is_format_macro (cx , macro_call . def_id) && let Some (format_args) = self . format_args . get (cx , expr , macro_call . expn) { let mut linter = FormatArgsExpr { cx , expr , macro_call : & macro_call , format_args , ignore_mixed : self . ignore_mixed , msrv : & self . msrv , ty_msrv_map : & self . ty_msrv_map , has_derived_debug : & mut self . has_derived_debug , has_pointer_format : & mut self . has_pointer_format , } ; linter . check_templates () ; if self . msrv . meets (cx , msrvs :: FORMAT_ARGS_CAPTURE) { linter . check_uninlined_args () ; } } } }
};
}
