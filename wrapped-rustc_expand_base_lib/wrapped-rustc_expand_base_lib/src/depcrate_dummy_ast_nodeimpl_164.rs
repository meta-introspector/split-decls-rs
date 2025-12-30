// Generated macro for impl_164 (impl)
macro_rules! Depcrate_dummy_ast_nodeimpl_164 {
() => {
// Module: crate::dummy_ast_node
// Provides: {"impl_164"}
// Dependencies: {}
impl DummyAstNode for AstNodeWrapper < ast :: Expr , MethodReceiverTag > { # [doc = " Returns a dummy `AstNodeWrapper` for a method receiver expression."] fn dummy () -> Self { AstNodeWrapper :: new (ast :: Expr :: dummy () , MethodReceiverTag) } }
};
}
