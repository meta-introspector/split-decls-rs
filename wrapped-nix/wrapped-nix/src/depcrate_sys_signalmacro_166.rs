// Generated macro for macro_166 (macro)
macro_rules! Depcrate_sys_signalmacro_166 {
() => {
// Module: crate::sys::signal
// Provides: {"macro_166"}
// Dependencies: {}
feature ! { #! [feature = "signal"] # [derive (Clone , Copy , Debug , Eq , Hash , PartialEq)] # [doc = " Iterate through all signals defined by this operating system"] pub struct SignalIterator { next : usize , } impl Iterator for SignalIterator { type Item = Signal ; fn next (& mut self) -> Option < Signal > { if self . next < SIGNALS . len () { let next_signal = SIGNALS [self . next] ; self . next += 1 ; Some (next_signal) } else { None } } } impl Signal { # [doc = " Iterate through all signals defined by this OS"] pub const fn iterator () -> SignalIterator { SignalIterator { next : 0 } } } # [doc = " Alias for [`SIGABRT`]"] pub const SIGIOT : Signal = SIGABRT ; # [doc = " Alias for [`SIGIO`]"] # [cfg (not (target_os = "haiku"))] pub const SIGPOLL : Signal = SIGIO ; # [doc = " Alias for [`SIGSYS`]"] pub const SIGUNUSED : Signal = SIGSYS ; cfg_if ! { if # [cfg (target_os = "redox")] { type SaFlags_t = libc :: c_ulong ; } else if # [cfg (target_env = "uclibc")] { type SaFlags_t = libc :: c_ulong ; } else { type SaFlags_t = libc :: c_int ; } } }
};
}
