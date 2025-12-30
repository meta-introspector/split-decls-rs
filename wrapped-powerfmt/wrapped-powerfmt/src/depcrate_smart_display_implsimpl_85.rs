// Generated macro for impl_85 (impl)
macro_rules! Depcrate_smart_display_implsimpl_85 {
() => {
// Module: crate::smart_display_impls
// Provides: {"impl_85"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < T > SmartDisplay for Rc < T > where T : SmartDisplay + ? Sized , { type Metadata = T :: Metadata ; fn metadata (& self , f : FormatterOptions) -> Metadata < '_ , Self > { (* * self) . metadata (f) . reuse () } fn fmt (& self , f : & mut Formatter < '_ >) -> fmt :: Result { SmartDisplay :: fmt (& * * self , f) } }
};
}
