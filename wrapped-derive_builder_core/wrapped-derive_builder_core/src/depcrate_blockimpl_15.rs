// Generated macro for impl_15 (impl)
macro_rules! Depcrate_blockimpl_15 {
() => {
// Module: crate::block
// Provides: {"impl_15"}
// Dependencies: {}
impl From < syn :: Expr > for BlockContents { fn from (v : syn :: Expr) -> Self { Self (Block { brace_token : syn :: token :: Brace (v . span ()) , stmts : vec ! [syn :: Stmt :: Expr (v , None)] , }) } }
};
}
