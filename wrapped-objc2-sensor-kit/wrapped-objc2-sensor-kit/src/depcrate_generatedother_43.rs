// Generated macro for other_43 (other)
macro_rules! Depcrate_generatedother_43 {
() => {
// Module: crate::generated
// Provides: {"other_43"}
// Dependencies: {}
extern "C-unwind" { # [doc = " Convert a CFAbsoluteTime to an SRAbsoluteTime."] # [doc = ""] # [doc = " The SRAbsoluteTime returned is based on calculations relative to the current"] # [doc = " wall clock. This means that if the system time is 5 seconds fast against UTC,"] # [doc = " the result will be 5 seconds fast to when the event happened relative to UTC."] # [cfg (feature = "objc2-core-foundation")] pub fn SRAbsoluteTimeFromCFAbsoluteTime (cf : CFAbsoluteTime) -> SRAbsoluteTime ; }
};
}
