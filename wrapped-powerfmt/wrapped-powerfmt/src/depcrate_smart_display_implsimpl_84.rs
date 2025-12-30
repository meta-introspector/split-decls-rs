// Generated macro for impl_84 (impl)
macro_rules! Depcrate_smart_display_implsimpl_84 {
() => {
// Module: crate::smart_display_impls
// Provides: {"impl_84"}
// Dependencies: {}
impl < T > SmartDisplay for Wrapping < T > where T : SmartDisplay , { type Metadata = T :: Metadata ; fn metadata (& self , f : FormatterOptions) -> Metadata < '_ , Self > { self . 0 . metadata (f) . reuse () } fn fmt (& self , f : & mut Formatter < '_ >) -> fmt :: Result { SmartDisplay :: fmt (& self . 0 , f) } }
};
}
