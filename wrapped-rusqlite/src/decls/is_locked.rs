macro_rules! is_locked {
    () => {
        pub unsafe fn is_locked (db : * mut ffi :: sqlite3 , rc : c_int) -> bool { rc == ffi :: SQLITE_LOCKED_SHAREDCACHE || (rc & 0xFF) == ffi :: SQLITE_LOCKED && ffi :: sqlite3_extended_errcode (db) == ffi :: SQLITE_LOCKED_SHAREDCACHE }
    };
}

is_locked!();