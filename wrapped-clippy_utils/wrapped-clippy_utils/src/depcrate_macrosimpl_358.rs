// Generated macro for impl_358 (impl)
macro_rules! Depcrate_macrosimpl_358 {
() => {
// Module: crate::macros
// Provides: {"impl_358"}
// Dependencies: {}
impl FormatArgsStorage { # [doc = " Returns an AST [`FormatArgs`] node if a `format_args` expansion is found as a descendant of"] # [doc = " `expn_id`"] # [doc = ""] # [doc = " See also [`find_format_arg_expr`]"] pub fn get (& self , cx : & LateContext < '_ > , start : & Expr < '_ > , expn_id : ExpnId) -> Option < & FormatArgs > { let format_args_expr = for_each_expr_without_closures (start , | expr | { let ctxt = expr . span . ctxt () ; if ctxt . outer_expn () . is_descendant_of (expn_id) { if macro_backtrace (expr . span) . map (| macro_call | cx . tcx . item_name (macro_call . def_id)) . any (| name | matches ! (name , sym :: const_format_args | sym :: format_args | sym :: format_args_nl)) { ControlFlow :: Break (expr) } else { ControlFlow :: Continue (Descend :: Yes) } } else { ControlFlow :: Continue (Descend :: No) } }) ? ; debug_assert ! (self . 0 . get () . is_some () , "`FormatArgsStorage` not yet populated") ; self . 0 . get () ? . get (& format_args_expr . span . with_parent (None)) } # [doc = " Should only be called by `FormatArgsCollector`"] pub fn set (& self , format_args : FxHashMap < Span , FormatArgs >) { self . 0 . set (format_args) . expect ("`FormatArgsStorage::set` should only be called once") ; } }
};
}
