// Generated macro for impl_661 (impl)
macro_rules! Depcrate_suggimpl_661 {
() => {
// Module: crate::sugg
// Provides: {"impl_661"}
// Dependencies: {}
impl < T : Display > Display for ParenHelper < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> Result < () , fmt :: Error > { if self . paren { write ! (f , "({})" , self . wrapped) } else { self . wrapped . fmt (f) } } }
};
}
