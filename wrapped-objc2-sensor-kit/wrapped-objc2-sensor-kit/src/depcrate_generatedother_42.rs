// Generated macro for other_42 (other)
macro_rules! Depcrate_generatedother_42 {
() => {
// Module: crate::generated
// Provides: {"other_42"}
// Dependencies: {}
extern "C-unwind" { # [doc = " Convert a SRAbsoluteTime to a CFAbsoluteTime."] # [doc = ""] # [doc = " The CFAbsoluteTime returned is based on calculations relative to the current"] # [doc = " wall clock. This means that if the system time is 5 seconds fast against UTC,"] # [doc = " the result will be 5 seconds fast to when the event happened relative to UTC."] # [cfg (feature = "objc2-core-foundation")] pub fn SRAbsoluteTimeToCFAbsoluteTime (sr : SRAbsoluteTime) -> CFAbsoluteTime ; }
};
}
