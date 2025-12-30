// Generated macro for impl_51 (impl)
macro_rules! Depcrate_displayimpl_51 {
() => {
// Module: crate::display
// Provides: {"impl_51"}
// Dependencies: {}
impl < 'a , I , S : 'a + ToOwned + ? Sized > From < I > for ANSIGenericString < 'a , S > where I : Into < Cow < 'a , S > > , < S as ToOwned > :: Owned : fmt :: Debug { fn from (input : I) -> ANSIGenericString < 'a , S > { ANSIGenericString { string : input . into () , style : Style :: default () , } } }
};
}
