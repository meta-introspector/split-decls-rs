// Generated macro for impl_746 (impl)
macro_rules! Depcrate_errnoimpl_746 {
() => {
// Module: crate::errno
// Provides: {"impl_746"}
// Dependencies: {}
impl ToErrno for i32 { fn to_errno (& self) -> Option < i32 > { (* self < 0) . then_some (- self) } }
};
}
