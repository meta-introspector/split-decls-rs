// Generated macro for impl_83 (impl)
macro_rules! Depcrate_smart_display_implsimpl_83 {
() => {
// Module: crate::smart_display_impls
// Provides: {"impl_83"}
// Dependencies: {}
impl < T > SmartDisplay for RefMut < '_ , T > where T : SmartDisplay + ? Sized , { type Metadata = T :: Metadata ; fn metadata (& self , f : FormatterOptions) -> Metadata < '_ , Self > { (* * self) . metadata (f) . reuse () } fn fmt (& self , f : & mut Formatter < '_ >) -> fmt :: Result { SmartDisplay :: fmt (& * * self , f) } }
};
}
