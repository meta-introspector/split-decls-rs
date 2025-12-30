// Generated macro for other_2236 (other)
macro_rules! Depcrate_generatedother_2236 {
() => {
// Module: crate::generated
// Provides: {"other_2236"}
// Dependencies: {}
extern "C-unwind" { # [doc = " Create a new clock object."] # [doc = ""] # [doc = ""] # [doc = " Parameter `inReservedFlags`: Must be 0."] # [doc = ""] # [doc = ""] # [doc = " Parameter `outCAClock`: Must be non-null. On successful return, the new clock object."] # [doc = ""] # [doc = ""] # [doc = " Returns: An OSStatus error code."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " `out_ca_clock` must be a valid pointer."] pub fn CAClockNew (in_reserved_flags : u32 , out_ca_clock : NonNull < CAClockRef >) -> OSStatus ; }
};
}
