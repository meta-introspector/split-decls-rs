// Generated macro for impl_982 (impl)
macro_rules! Depcrate_util_sqlite_stringimpl_982 {
() => {
// Module: crate::util::sqlite_string
// Provides: {"impl_982"}
// Dependencies: {}
impl Drop for SqliteMallocString { # [inline] fn drop (& mut self) { unsafe { ffi :: sqlite3_free (self . ptr . as_ptr () . cast ()) } ; } }
};
}
