// Generated macro for impl_1575 (impl)
macro_rules! Depcrate_syscalls_interfaces_uhyveimpl_1575 {
() => {
// Module: crate::syscalls::interfaces::uhyve
// Provides: {"impl_1575"}
// Dependencies: {}
impl SyscallInterface for Uhyve { fn shutdown (& self , error_code : i32) -> ! { let sysexit = ExitParams { arg : error_code } ; uhyve_hypercall (Hypercall :: Exit (& sysexit)) ; loop { arch :: processor :: halt () ; } } }
};
}
