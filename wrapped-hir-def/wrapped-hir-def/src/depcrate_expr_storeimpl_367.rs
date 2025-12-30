// Generated macro for impl_367 (impl)
macro_rules! Depcrate_expr_storeimpl_367 {
() => {
// Module: crate::expr_store
// Provides: {"impl_367"}
// Dependencies: {}
impl HygieneId { pub const ROOT : Self = Self (span :: SyntaxContext :: root (Edition :: Edition2015)) ; pub fn new (mut ctx : span :: SyntaxContext) -> Self { ctx . remove_root_edition () ; Self (ctx) } pub (crate) fn lookup (self) -> SyntaxContext { self . 0 } pub (crate) fn is_root (self) -> bool { self . 0 . is_root () } }
};
}
