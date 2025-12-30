// Generated macro for impl_415 (impl)
macro_rules! Depcrate_common_hostimpl_415 {
() => {
// Module: crate::common::host
// Provides: {"impl_415"}
// Dependencies: {}
impl Host { # [doc = " Get the hostname, such as example.domain."] pub fn hostname (& self) -> & str { self . 0 . host () } # [doc = " Get the optional port number."] pub fn port (& self) -> Option < u16 > { self . 0 . port_u16 () } }
};
}
