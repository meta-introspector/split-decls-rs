// Generated macro for other_2246 (other)
macro_rules! Depcrate_generatedother_2246 {
() => {
// Module: crate::generated
// Provides: {"other_2246"}
// Dependencies: {}
extern "C-unwind" { # [doc = " Disallow received sync messages from starting the clock."] # [doc = ""] # [doc = ""] # [doc = " Parameter `inCAClock`: The clock object."] # [doc = ""] # [doc = ""] # [doc = " Returns: An OSStatus error code."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " `in_ca_clock` must be a valid pointer."] pub fn CAClockDisarm (in_ca_clock : CAClockRef) -> OSStatus ; }
};
}
