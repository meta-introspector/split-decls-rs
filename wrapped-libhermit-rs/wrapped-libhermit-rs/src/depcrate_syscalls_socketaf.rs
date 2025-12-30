// Generated macro for Af (enum)
macro_rules! Depcrate_syscalls_socketAf {
() => {
// Module: crate::syscalls::socket
// Provides: {"Af"}
// Dependencies: {}
# [derive (TryFromPrimitive , IntoPrimitive , PartialEq , Eq , Clone , Copy , Debug)] # [repr (u8)] pub enum Af { Unspec = 0 , Inet = 3 , Inet6 = 1 , Unix = 4 , # [cfg (feature = "vsock")] Vsock = 2 , }
};
}
