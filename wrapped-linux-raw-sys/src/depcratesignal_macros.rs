// Generated macro for signal_macros (module)
macro_rules! Depcratesignal_macros {
() => {
// Module: crate
// Provides: {"signal_macros"}
// Dependencies: {}
# [cfg (feature = "general")] pub mod signal_macros { pub const SIG_DFL : super :: general :: __kernel_sighandler_t = None ; # [doc = " Rust doesn't currently permit us to use `transmute` to convert the"] # [doc = " `SIG_IGN` value into a function pointer in a `const` initializer, so"] # [doc = " we make it a function instead."] # [doc = ""] # [inline] pub const fn sig_ign () -> super :: general :: __kernel_sighandler_t { Some (unsafe { core :: mem :: transmute :: < usize , unsafe extern "C" fn (crate :: ctypes :: c_int) > (1) }) } }
};
}
