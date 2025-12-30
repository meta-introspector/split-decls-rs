// Generated macro for impl_60 (impl)
macro_rules! Depcrate_displayimpl_60 {
() => {
// Module: crate::display
// Provides: {"impl_60"}
// Dependencies: {}
impl Colour { # [doc = " Paints the given text with this colour, returning an ANSI string."] # [doc = " This is a short-cut so you don’t have to use `Blue.normal()` just"] # [doc = " to get blue text."] # [doc = ""] # [doc = " ```"] # [doc = " use ansiterm::Colour::Blue;"] # [doc = " println!(\"{}\", Blue.paint(\"da ba dee\"));"] # [doc = " ```"] # [must_use] pub fn paint < 'a , I , S : 'a + ToOwned + ? Sized > (self , input : I) -> ANSIGenericString < 'a , S > where I : Into < Cow < 'a , S > > , < S as ToOwned > :: Owned : fmt :: Debug { ANSIGenericString { string : input . into () , style : self . normal () , } } }
};
}
