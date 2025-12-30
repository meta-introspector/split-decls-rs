// Generated macro for impl_468 (impl)
macro_rules! Depcrate_duration_formatimpl_468 {
() => {
// Module: crate::duration::format
// Provides: {"impl_468"}
// Dependencies: {}
impl Writeable for FormattedDuration < '_ > { fn write_to_parts < S : PartsWrite + ? Sized > (& self , sink : & mut S) -> fmt :: Result { self . partition_duration_format_pattern (sink) } }
};
}
