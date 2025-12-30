// Generated macro for SQLITE_TRANSIENT (function)
macro_rules! DepcrateSQLITE_TRANSIENT {
() => {
// Module: crate
// Provides: {"SQLITE_TRANSIENT"}
// Dependencies: {}
# [must_use] pub fn SQLITE_TRANSIENT () -> sqlite3_destructor_type { Some (unsafe { mem :: transmute :: < isize , unsafe extern "C" fn (* mut std :: ffi :: c_void) > (- 1_isize) }) }
};
}
