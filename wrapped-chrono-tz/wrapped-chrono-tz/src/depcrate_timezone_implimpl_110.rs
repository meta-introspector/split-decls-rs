// Generated macro for impl_110 (impl)
macro_rules! Depcrate_timezone_implimpl_110 {
() => {
// Module: crate::timezone_impl
// Provides: {"impl_110"}
// Dependencies: {}
impl Offset for TzOffset { fn fix (& self) -> FixedOffset { FixedOffset :: east_opt (self . offset . offset) . unwrap () } }
};
}
