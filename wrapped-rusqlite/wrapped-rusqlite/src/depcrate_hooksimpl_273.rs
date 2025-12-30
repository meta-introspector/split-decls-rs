// Generated macro for impl_273 (impl)
macro_rules! Depcrate_hooksimpl_273 {
() => {
// Module: crate::hooks
// Provides: {"impl_273"}
// Dependencies: {}
impl Authorization { fn into_raw (self) -> c_int { match self { Self :: Allow => ffi :: SQLITE_OK , Self :: Ignore => ffi :: SQLITE_IGNORE , Self :: Deny => ffi :: SQLITE_DENY , } } }
};
}
