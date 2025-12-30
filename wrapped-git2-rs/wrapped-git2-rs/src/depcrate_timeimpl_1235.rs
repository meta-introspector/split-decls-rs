// Generated macro for impl_1235 (impl)
macro_rules! Depcrate_timeimpl_1235 {
() => {
// Module: crate::time
// Provides: {"impl_1235"}
// Dependencies: {}
impl Binding for Time { type Raw = raw :: git_time ; unsafe fn from_raw (raw : raw :: git_time) -> Time { Time { raw } } fn raw (& self) -> raw :: git_time { self . raw } }
};
}
