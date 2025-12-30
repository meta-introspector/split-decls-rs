// Generated macro for impl_79 (impl)
macro_rules! Depcrate_log_formatimpl_79 {
() => {
// Module: crate::log::format
// Provides: {"impl_79"}
// Dependencies: {}
impl LogMetadata { # [doc = " Checks whether this `LogMetadata` came from a specifier such as"] # [doc = " {t}, {f}, etc."] fn is_metadata_specifier (& self) -> bool { ! matches ! (self , LogMetadata :: String (_) | LogMetadata :: NestedLogSegments (_)) } }
};
}
