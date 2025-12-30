// Generated macro for impl_58 (impl)
macro_rules! Depcrate_testimpl_58 {
() => {
// Module: crate::test
// Provides: {"impl_58"}
// Dependencies: {}
impl IsAwait for Stmt { fn is_await (& self) -> bool { match self { Stmt :: Expr (Expr :: Await (_) , _) => true , _ => false , } } }
};
}
