// Generated macro for impl_59 (impl)
macro_rules! Depcrate_displayimpl_59 {
() => {
// Module: crate::display
// Provides: {"impl_59"}
// Dependencies: {}
impl Style { # [doc = " Paints the given text with this colour, returning an ANSI string."] # [must_use] pub fn paint < 'a , I , S : 'a + ToOwned + ? Sized > (self , input : I) -> ANSIGenericString < 'a , S > where I : Into < Cow < 'a , S > > , < S as ToOwned > :: Owned : fmt :: Debug { ANSIGenericString { string : input . into () , style : self , } } }
};
}
