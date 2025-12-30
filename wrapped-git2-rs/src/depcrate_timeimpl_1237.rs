// Generated macro for impl_1237 (impl)
macro_rules! Depcrate_timeimpl_1237 {
() => {
// Module: crate::time
// Provides: {"impl_1237"}
// Dependencies: {}
impl Binding for IndexTime { type Raw = raw :: git_index_time ; unsafe fn from_raw (raw : raw :: git_index_time) -> IndexTime { IndexTime { raw } } fn raw (& self) -> raw :: git_index_time { self . raw } }
};
}
