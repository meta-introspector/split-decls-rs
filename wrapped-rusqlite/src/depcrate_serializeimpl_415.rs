// Generated macro for impl_415 (impl)
macro_rules! Depcrate_serializeimpl_415 {
() => {
// Module: crate::serialize
// Provides: {"impl_415"}
// Dependencies: {}
impl Drop for OwnedData { fn drop (& mut self) { unsafe { ffi :: sqlite3_free (self . ptr . as_ptr () . cast ()) ; } } }
};
}
