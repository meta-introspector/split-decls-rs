// Generated macro for AstNodeWrapper (struct)
macro_rules! Depcrate_ast_traitsAstNodeWrapper {
() => {
// Module: crate::ast_traits
// Provides: {"AstNodeWrapper"}
// Dependencies: {}
# [doc = " A newtype around an AST node that implements the traits above if the node implements them."] # [repr (transparent)] pub struct AstNodeWrapper < Wrapped , Tag > { pub wrapped : Wrapped , pub tag : PhantomData < Tag > , }
};
}
