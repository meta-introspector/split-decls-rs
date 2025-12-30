// Generated macro for impl_192 (impl)
macro_rules! Depcrate_astimpl_192 {
() => {
// Module: crate::ast
// Provides: {"impl_192"}
// Dependencies: {}
impl fmt :: Debug for ImplPolarity { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self { ImplPolarity :: Positive => "positive" . fmt (f) , ImplPolarity :: Negative (_) => "negative" . fmt (f) , } } }
};
}
