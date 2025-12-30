// Generated macro for impl_2265 (impl)
macro_rules! Depcrate_mysql_connection_rawimpl_2265 {
() => {
// Module: crate::mysql::connection::raw
// Provides: {"impl_2265"}
// Dependencies: {}
impl Drop for RawConnection { fn drop (& mut self) { unsafe { ffi :: mysql_close (self . 0 . as_ptr ()) ; } } }
};
}
