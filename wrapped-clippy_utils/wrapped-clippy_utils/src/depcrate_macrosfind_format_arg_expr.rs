// Generated macro for find_format_arg_expr (function)
macro_rules! Depcrate_macrosfind_format_arg_expr {
() => {
// Module: crate::macros
// Provides: {"find_format_arg_expr"}
// Dependencies: {}
# [doc = " Attempt to find the [`rustc_hir::Expr`] that corresponds to the [`FormatArgument`]'s value"] pub fn find_format_arg_expr < 'hir > (start : & 'hir Expr < 'hir > , target : & FormatArgument) -> Option < & 'hir Expr < 'hir > > { let SpanData { lo , hi , ctxt , parent : _ , } = target . expr . span . data () ; for_each_expr_without_closures (start , | expr | { let data = expr . span . data () ; if data . lo == lo && data . hi == hi && data . ctxt == ctxt { ControlFlow :: Break (expr) } else { ControlFlow :: Continue (()) } }) }
};
}
