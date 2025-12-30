// Generated macro for impl_52 (impl)
macro_rules! Depcrate_displayimpl_52 {
() => {
// Module: crate::display
// Provides: {"impl_52"}
// Dependencies: {}
impl < 'a , S : 'a + ToOwned + ? Sized > ANSIGenericString < 'a , S > where < S as ToOwned > :: Owned : fmt :: Debug { # [doc = " Directly access the style"] pub fn style_ref (& self) -> & Style { & self . style } # [doc = " Directly access the style mutably"] pub fn style_ref_mut (& mut self) -> & mut Style { & mut self . style } # [doc = " Directly access the string"] pub fn as_str (& self) -> & S { self . string . as_ref () } }
};
}
