// Generated macro for slow_platform (function)
macro_rules! Depcrate_run_cfgslow_platform {
() => {
// Module: crate::run_cfg
// Provides: {"slow_platform"}
// Dependencies: {}
# [doc = " Tests are pretty slow on non-64-bit targets, x86 MacOS, and targets that run in QEMU. Start"] # [doc = " with a reduced number on these platforms."] fn slow_platform () -> bool { let slow_on_ci = crate :: emulated () || usize :: BITS < 64 || cfg ! (all (target_arch = "x86_64" , target_vendor = "apple")) ; slow_on_ci && crate :: ci () }
};
}
