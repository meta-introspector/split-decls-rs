// Generated macro for impl_265 (impl)
macro_rules! Depcrate_hooksimpl_265 {
() => {
// Module: crate::hooks
// Provides: {"impl_265"}
// Dependencies: {}
impl From < i32 > for Action { # [inline] fn from (code : i32) -> Self { match code { ffi :: SQLITE_DELETE => Self :: SQLITE_DELETE , ffi :: SQLITE_INSERT => Self :: SQLITE_INSERT , ffi :: SQLITE_UPDATE => Self :: SQLITE_UPDATE , _ => Self :: UNKNOWN , } } }
};
}
