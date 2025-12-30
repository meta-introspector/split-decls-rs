// Generated macro for peel_non_expn_blocks (function)
macro_rules! Depcrate_methods_format_collectpeel_non_expn_blocks {
() => {
// Module: crate::methods::format_collect
// Provides: {"peel_non_expn_blocks"}
// Dependencies: {}
# [doc = " Same as `peel_blocks` but only actually considers blocks that are not from an expansion."] # [doc = " This is needed because always calling `peel_blocks` would otherwise remove parts of the"] # [doc = " `format!` macro, which would cause `root_macro_call_first_node` to return `None`."] fn peel_non_expn_blocks < 'tcx > (expr : & 'tcx Expr < 'tcx >) -> Option < & 'tcx Expr < 'tcx > > { match expr . kind { ExprKind :: Block (block , _) if ! expr . span . from_expansion () => peel_non_expn_blocks (block . expr ?) , _ => Some (expr) , } }
};
}
