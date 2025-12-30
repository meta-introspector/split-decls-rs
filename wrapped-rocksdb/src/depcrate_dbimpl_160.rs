// Generated macro for impl_160 (impl)
macro_rules! Depcrate_dbimpl_160 {
() => {
// Module: crate::db
// Provides: {"impl_160"}
// Dependencies: {}
impl < T : ThreadMode , I : DBInner > Drop for DBCommon < T , I > { fn drop (& mut self) { self . cfs . drop_all_cfs_internal () ; } }
};
}
