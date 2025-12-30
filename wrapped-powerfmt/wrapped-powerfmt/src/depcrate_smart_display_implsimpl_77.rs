// Generated macro for impl_77 (impl)
macro_rules! Depcrate_smart_display_implsimpl_77 {
() => {
// Module: crate::smart_display_impls
// Provides: {"impl_77"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl SmartDisplay for String { type Metadata = () ; # [inline] fn metadata (& self , f : FormatterOptions) -> Metadata < '_ , Self > { (* * self) . metadata (f) . reuse () } # [inline] fn fmt (& self , f : & mut Formatter < '_ >) -> fmt :: Result { Display :: fmt (self , f) } }
};
}
