// Generated macro for impl_53 (impl)
macro_rules! Depcrate_displayimpl_53 {
() => {
// Module: crate::display
// Provides: {"impl_53"}
// Dependencies: {}
impl < 'a , S : 'a + ToOwned + ? Sized > Deref for ANSIGenericString < 'a , S > where < S as ToOwned > :: Owned : fmt :: Debug { type Target = S ; fn deref (& self) -> & S { self . string . deref () } }
};
}
