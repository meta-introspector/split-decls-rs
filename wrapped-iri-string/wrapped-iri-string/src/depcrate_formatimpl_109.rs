// Generated macro for impl_109 (impl)
macro_rules! Depcrate_formatimpl_109 {
() => {
// Module: crate::format
// Provides: {"impl_109"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < T : fmt :: Display > ToStringFallible for T { # [doc = " [`ToString::to_string`][`alloc::string::ToString::to_string`], but without panic on OOM."] # [inline] fn try_to_string (& self) -> Result < String , TryReserveError > { let mut buf = String :: new () ; try_append_to_string (& mut buf , self) ? ; Ok (buf) } }
};
}
