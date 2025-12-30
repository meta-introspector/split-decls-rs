// Generated macro for impl_57 (impl)
macro_rules! Depcrate_displayimpl_57 {
() => {
// Module: crate::display
// Provides: {"impl_57"}
// Dependencies: {}
impl Color { # [doc = " Paints the given text with this color, returning an ANSI string."] # [doc = " This is a short-cut so you don’t have to use `Blue.normal()` just"] # [doc = " to get blue text."] # [doc = ""] # [doc = " ```"] # [doc = " use nu_ansi_term::Color::Blue;"] # [doc = " println!(\"{}\", Blue.paint(\"da ba dee\"));"] # [doc = " ```"] # [must_use] pub fn paint < 'a , I , S : 'a + ToOwned + ? Sized > (self , input : I) -> AnsiGenericString < 'a , S > where I : Into < Cow < 'a , S > > , < S as ToOwned > :: Owned : fmt :: Debug , { AnsiGenericString { string : input . into () , style : self . normal () , oscontrol : None , } } }
};
}
