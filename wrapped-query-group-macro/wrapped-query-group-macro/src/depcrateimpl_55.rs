// Generated macro for impl_55 (impl)
macro_rules! Depcrateimpl_55 {
() => {
// Module: crate
// Provides: {"impl_55"}
// Dependencies: {}
impl VisitMut for SelfToDbRewriter { fn visit_expr_path_mut (& mut self , i : & mut syn :: ExprPath) { if i . path . is_ident ("self") { i . path = parse_quote_spanned ! (i . path . span () => db) ; } } }
};
}
