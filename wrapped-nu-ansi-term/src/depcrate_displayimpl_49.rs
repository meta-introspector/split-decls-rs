// Generated macro for impl_49 (impl)
macro_rules! Depcrate_displayimpl_49 {
() => {
// Module: crate::display
// Provides: {"impl_49"}
// Dependencies: {}
impl < 'a , I , S : 'a + ToOwned + ? Sized > From < I > for AnsiGenericString < 'a , S > where I : Into < Cow < 'a , S > > , < S as ToOwned > :: Owned : fmt :: Debug , { fn from (input : I) -> AnsiGenericString < 'a , S > { AnsiGenericString { string : input . into () , style : Style :: default () , oscontrol : None , } } }
};
}
