// Generated macro for impl_56 (impl)
macro_rules! Depcrate_displayimpl_56 {
() => {
// Module: crate::display
// Provides: {"impl_56"}
// Dependencies: {}
impl Style { # [doc = " Paints the given text with this color, returning an ANSI string."] # [must_use] pub fn paint < 'a , I , S : 'a + ToOwned + ? Sized > (self , input : I) -> AnsiGenericString < 'a , S > where I : Into < Cow < 'a , S > > , < S as ToOwned > :: Owned : fmt :: Debug , { AnsiGenericString { string : input . into () , style : self , oscontrol : None , } } }
};
}
