// Generated macro for impl_79 (impl)
macro_rules! Depcrate_smart_display_implsimpl_79 {
() => {
// Module: crate::smart_display_impls
// Provides: {"impl_79"}
// Dependencies: {}
impl < T > SmartDisplay for Pin < & T > where T : SmartDisplay + ? Sized , { type Metadata = T :: Metadata ; fn metadata (& self , f : FormatterOptions) -> Metadata < '_ , Self > { self . get_ref () . metadata (f) . reuse () } fn fmt (& self , f : & mut Formatter < '_ >) -> fmt :: Result { SmartDisplay :: fmt (self . get_ref () , f) } }
};
}
