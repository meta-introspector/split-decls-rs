// Generated macro for impl_292 (impl)
macro_rules! Depcrate_ast_traitsimpl_292 {
() => {
// Module: crate::ast_traits
// Provides: {"impl_292"}
// Dependencies: {}
impl < T , Tag > From < AstNodeWrapper < Box < T > , Tag > > for AstNodeWrapper < T , Tag > { fn from (value : AstNodeWrapper < Box < T > , Tag >) -> Self { AstNodeWrapper { wrapped : * value . wrapped , tag : value . tag } } }
};
}
