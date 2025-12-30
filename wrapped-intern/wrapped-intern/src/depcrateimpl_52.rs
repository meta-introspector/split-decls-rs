// Generated macro for impl_52 (impl)
macro_rules! Depcrateimpl_52 {
() => {
// Module: crate
// Provides: {"impl_52"}
// Dependencies: {}
impl < T : Debug + Internable + ? Sized > Debug for Interned < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { (* self . arc) . fmt (f) } }
};
}
