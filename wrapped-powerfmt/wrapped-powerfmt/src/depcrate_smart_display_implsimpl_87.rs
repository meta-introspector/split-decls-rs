// Generated macro for impl_87 (impl)
macro_rules! Depcrate_smart_display_implsimpl_87 {
() => {
// Module: crate::smart_display_impls
// Provides: {"impl_87"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < T > SmartDisplay for Box < T > where T : SmartDisplay + ? Sized , { type Metadata = T :: Metadata ; fn metadata (& self , f : FormatterOptions) -> Metadata < '_ , Self > { (* * self) . metadata (f) . reuse () } fn fmt (& self , f : & mut Formatter < '_ >) -> fmt :: Result { SmartDisplay :: fmt (& * * self , f) } }
};
}
