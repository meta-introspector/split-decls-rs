// Generated macro for impl_747 (impl)
macro_rules! Depcrate_errnoimpl_747 {
() => {
// Module: crate::errno
// Provides: {"impl_747"}
// Dependencies: {}
impl ToErrno for i64 { fn to_errno (& self) -> Option < i32 > { (* self < 0) . then (| | i32 :: try_from (- self) . unwrap ()) } }
};
}
