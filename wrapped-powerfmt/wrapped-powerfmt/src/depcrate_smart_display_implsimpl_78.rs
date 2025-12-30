// Generated macro for impl_78 (impl)
macro_rules! Depcrate_smart_display_implsimpl_78 {
() => {
// Module: crate::smart_display_impls
// Provides: {"impl_78"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < 'a , B , O > SmartDisplay for Cow < 'a , B > where B : SmartDisplay + ToOwned < Owned = O > + ? Sized , O : SmartDisplay < Metadata = B :: Metadata > + 'a , { type Metadata = B :: Metadata ; fn metadata (& self , f : FormatterOptions) -> Metadata < '_ , Self > { match * self { Cow :: Borrowed (ref b) => b . metadata (f) . reuse () , Cow :: Owned (ref o) => o . metadata (f) . reuse () , } } fn fmt (& self , f : & mut Formatter < '_ >) -> fmt :: Result { Display :: fmt (self , f) } }
};
}
