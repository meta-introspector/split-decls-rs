// Generated macro for is_locked (function)
macro_rules! Depcrate_unlock_notifyis_locked {
() => {
// Module: crate::unlock_notify
// Provides: {"is_locked"}
// Dependencies: {}
pub unsafe fn is_locked (db : * mut ffi :: sqlite3 , rc : c_int) -> bool { rc == ffi :: SQLITE_LOCKED_SHAREDCACHE || (rc & 0xFF) == ffi :: SQLITE_LOCKED && ffi :: sqlite3_extended_errcode (db) == ffi :: SQLITE_LOCKED_SHAREDCACHE }
};
}
