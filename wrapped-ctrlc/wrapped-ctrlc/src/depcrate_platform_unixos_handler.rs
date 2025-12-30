// Generated macro for os_handler (function)
macro_rules! Depcrate_platform_unixos_handler {
() => {
// Module: crate::platform::unix
// Provides: {"os_handler"}
// Dependencies: {}
extern "C" fn os_handler (_ : nix :: libc :: c_int) { unsafe { implementation :: sem_post () ; } }
};
}
