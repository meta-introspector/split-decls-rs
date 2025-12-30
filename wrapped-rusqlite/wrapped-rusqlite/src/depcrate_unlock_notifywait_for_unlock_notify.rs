// Generated macro for wait_for_unlock_notify (function)
macro_rules! Depcrate_unlock_notifywait_for_unlock_notify {
() => {
// Module: crate::unlock_notify
// Provides: {"wait_for_unlock_notify"}
// Dependencies: {}
# [doc = " This function assumes that an SQLite API call (either `sqlite3_prepare_v2()`"] # [doc = " or `sqlite3_step()`) has just returned `SQLITE_LOCKED`. The argument is the"] # [doc = " associated database connection."] # [doc = ""] # [doc = " This function calls `sqlite3_unlock_notify()` to register for an"] # [doc = " unlock-notify callback, then blocks until that callback is delivered"] # [doc = " and returns `SQLITE_OK`. The caller should then retry the failed operation."] # [doc = ""] # [doc = " Or, if `sqlite3_unlock_notify()` indicates that to block would deadlock"] # [doc = " the system, then this function returns `SQLITE_LOCKED` immediately. In"] # [doc = " this case the caller should not retry the operation and should roll"] # [doc = " back the current transaction (if any)."] # [cfg (feature = "unlock_notify")] pub unsafe fn wait_for_unlock_notify (db : * mut ffi :: sqlite3) -> c_int { let un = UnlockNotification :: new () ; let rc = ffi :: sqlite3_unlock_notify (db , Some (unlock_notify_cb) , & un as * const UnlockNotification as * mut c_void ,) ; debug_assert ! (rc == ffi :: SQLITE_LOCKED || rc == ffi :: SQLITE_LOCKED_SHAREDCACHE || rc == ffi :: SQLITE_OK) ; if rc == ffi :: SQLITE_OK { un . wait () ; } rc }
};
}
