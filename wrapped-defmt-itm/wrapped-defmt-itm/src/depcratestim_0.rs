// Generated macro for stim_0 (function)
macro_rules! Depcratestim_0 {
() => {
// Module: crate
// Provides: {"stim_0"}
// Dependencies: {}
# [doc = " Get access to stimulus port 0"] # [doc = ""] # [doc = " # Safety"] # [doc = " Can only be invoked *after* `enable` has run"] unsafe fn stim_0 < 'a > () -> & 'a mut Stim { & mut (* ITM :: PTR) . stim [0] }
};
}
