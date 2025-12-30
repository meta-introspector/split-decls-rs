// Generated macro for impl_10446 (impl)
macro_rules! Depcrate_unit_types_let_unit_valueimpl_10446 {
() => {
// Module: crate::unit_types::let_unit_value
// Provides: {"impl_10446"}
// Dependencies: {}
# [doc = "\n * Collect all instances where a variable is used based on its `HirId`.\n "] impl < 'tcx > Visitor < 'tcx > for UnitVariableCollector < '_ , 'tcx > { fn visit_expr (& mut self , ex : & 'tcx Expr < 'tcx >) -> Self :: Result { if let Some (macro_call) = root_macro_call_first_node (self . cx , ex) && is_format_macro (self . cx , macro_call . def_id) && let Some (format_args) = self . format_args . get (self . cx , ex , macro_call . expn) { let parent_macro_call = self . macro_call ; self . macro_call = Some (format_args) ; walk_expr (self , ex) ; self . macro_call = parent_macro_call ; return ; } if let ExprKind :: Path (QPath :: Resolved (None , path)) = ex . kind && let Res :: Local (id) = path . res && id == self . id { if let Some (macro_call) = self . macro_call && macro_call . arguments . all_args () . iter () . any (| arg | { matches ! (arg . kind , FormatArgumentKind :: Captured (_)) && find_format_arg_expr (ex , arg) . is_some () }) { self . spans . push (MaybeInFormatCapture :: Yes) ; } else { self . spans . push (MaybeInFormatCapture :: No (path . span)) ; } } walk_expr (self , ex) ; } }
};
}
