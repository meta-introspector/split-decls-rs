// Generated macro for other_2245 (other)
macro_rules! Depcrate_generatedother_2245 {
() => {
// Module: crate::generated
// Provides: {"other_2245"}
// Dependencies: {}
extern "C-unwind" { # [doc = " Allow received sync messages to start the clock."] # [doc = ""] # [doc = " If a clock is following and being controlled by an external transport"] # [doc = " (e.g. MIDI Time Code), call this to indicate that the client is ready to"] # [doc = " start its transport in response to the external transport having started."] # [doc = ""] # [doc = " The external time source will set the clock's start position and start"] # [doc = " the clock."] # [doc = ""] # [doc = ""] # [doc = " Parameter `inCAClock`: The clock object."] # [doc = ""] # [doc = ""] # [doc = " Returns: An OSStatus error code."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " `in_ca_clock` must be a valid pointer."] pub fn CAClockArm (in_ca_clock : CAClockRef) -> OSStatus ; }
};
}
