// Generated macro for impl_14 (impl)
macro_rules! Depcrate_expanderimpl_14 {
() => {
// Module: crate::expander
// Provides: {"impl_14"}
// Dependencies: {}
impl Fragment < '_ > { fn is_empty (& self) -> bool { match self { Fragment :: Empty => true , Fragment :: Tokens (it) => it . len () == 0 , Fragment :: Expr (it) => it . len () == 0 , Fragment :: Path (it) => it . len () == 0 , Fragment :: TokensOwned (it) => it . 0 . is_empty () , } } }
};
}
