// Generated macro for impl_10105 (impl)
macro_rules! Depcrate_swapimpl_10105 {
() => {
// Module: crate::swap
// Provides: {"impl_10105"}
// Dependencies: {}
impl < 'tcx > Visitor < 'tcx > for IndexBindingVisitor { fn visit_path_segment (& mut self , path_segment : & 'tcx rustc_hir :: PathSegment < 'tcx >) -> Self :: Result { if path_segment . ident == self . idx { self . found_used = true ; } } fn visit_expr (& mut self , expr : & 'tcx Expr < 'tcx >) -> Self :: Result { if expr . span . hi () <= self . suggest_span . hi () { return ; } match expr . kind { ExprKind :: Path (QPath :: Resolved (_ , path)) => { for segment in path . segments { self . visit_path_segment (segment) ; } } , _ => walk_expr (self , expr) , } } }
};
}
