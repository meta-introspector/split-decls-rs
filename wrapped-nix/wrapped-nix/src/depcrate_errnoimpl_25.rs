// Generated macro for impl_25 (impl)
macro_rules! Depcrate_errnoimpl_25 {
() => {
// Module: crate::errno
// Provides: {"impl_25"}
// Dependencies: {}
impl From < Errno > for io :: Error { fn from (err : Errno) -> Self { io :: Error :: from_raw_os_error (err as i32) } }
};
}
