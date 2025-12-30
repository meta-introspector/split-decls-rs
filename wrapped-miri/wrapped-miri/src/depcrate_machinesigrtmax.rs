// Generated macro for SIGRTMAX (const)
macro_rules! Depcrate_machineSIGRTMAX {
() => {
// Module: crate::machine
// Provides: {"SIGRTMAX"}
// Dependencies: {}
# [doc = " Last real-time signal."] # [doc = " `signal(7)` says it must be between 32 and 64 and specifies"] # [doc = " `SIGRTMAX` - `SIGRTMIN` >= 8 (which is the value of `_POSIX_RTSIG_MAX`)"] pub const SIGRTMAX : i32 = 42 ;
};
}
