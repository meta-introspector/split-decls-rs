// Generated macro for impl_44 (impl)
macro_rules! Depcrate_displayimpl_44 {
() => {
// Module: crate::display
// Provides: {"impl_44"}
// Dependencies: {}
impl < 'a , S : 'a + ToOwned + ? Sized > Clone for OSControl < 'a , S > where < S as ToOwned > :: Owned : fmt :: Debug , { fn clone (& self) -> Self { match self { Self :: Link { url : u } => Self :: Link { url : u . clone () } , Self :: Title => Self :: Title , } } }
};
}
