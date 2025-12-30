// Generated macro for impl_694 (impl)
macro_rules! Depcrate_suggimpl_694 {
() => {
// Module: crate::sugg
// Provides: {"impl_694"}
// Dependencies: {}
impl < T : Display > Display for ParenHelper < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> Result < () , fmt :: Error > { if self . paren { write ! (f , "({})" , self . wrapped) } else { self . wrapped . fmt (f) } } }
};
}
