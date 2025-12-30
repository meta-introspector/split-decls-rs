// Generated macro for ToStringFallible (trait)
macro_rules! Depcrate_formatToStringFallible {
() => {
// Module: crate::format
// Provides: {"ToStringFallible"}
// Dependencies: {}
# [doc = " [`ToString`][`alloc::string::ToString`], but without panic."] # [cfg (feature = "alloc")] pub trait ToStringFallible : alloc :: string :: ToString { # [doc = " [`ToString::to_string`][`alloc::string::ToString::to_string`], but without panic on OOM."] fn try_to_string (& self) -> Result < String , TryReserveError > ; }
};
}
