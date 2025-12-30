// Generated macro for version (function)
macro_rules! Depcrate_versionversion {
() => {
// Module: crate::version
// Provides: {"version"}
// Dependencies: {}
# [doc = " Returns the SQLite version as a string; e.g., `\"3.16.2\"` for version 3.16.2."] # [doc = ""] # [doc = " See [`sqlite3_libversion()`](https://www.sqlite.org/c3ref/libversion.html)."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics when version is not valid UTF-8."] # [inline] # [must_use] pub fn version () -> & 'static str { let cstr = unsafe { CStr :: from_ptr (ffi :: sqlite3_libversion ()) } ; cstr . to_str () . expect ("SQLite version string is not valid UTF8 ?!") }
};
}
