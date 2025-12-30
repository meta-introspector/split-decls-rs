// Generated macro for impl_748 (impl)
macro_rules! Depcrate_errnoimpl_748 {
() => {
// Module: crate::errno
// Provides: {"impl_748"}
// Dependencies: {}
impl ToErrno for isize { fn to_errno (& self) -> Option < i32 > { (* self < 0) . then (| | i32 :: try_from (- self) . unwrap ()) } }
};
}
