// Generated macro for min_sigstack_size (function)
macro_rules! Depcrate_signal_handlermin_sigstack_size {
() => {
// Module: crate::signal_handler
// Provides: {"min_sigstack_size"}
// Dependencies: {}
# [doc = " Not all OS support hardware where this is needed."] # [cfg (not (any (target_os = "linux" , target_os = "android")))] fn min_sigstack_size () -> usize { libc :: MINSIGSTKSZ }
};
}
