// Generated macro for SDuration (enum)
macro_rules! Depcrate_durationSDuration {
() => {
// Module: crate::duration
// Provides: {"SDuration"}
// Dependencies: {}
# [doc = " An internal type for abstracting over signed durations."] # [doc = ""] # [doc = " This is typically converted to from a `Duration`. It enables callers"] # [doc = " downstream to implement datetime arithmetic on only two duration types"] # [doc = " instead of doing it for three duration types (including"] # [doc = " `std::time::Duration`)."] # [doc = ""] # [doc = " The main thing making this idea work is that if an unsigned duration cannot"] # [doc = " fit into a signed duration, then it would overflow any calculation on any"] # [doc = " datetime type in Jiff anyway. If this weren't true, then we'd need to"] # [doc = " support doing actual arithmetic with unsigned durations separately from"] # [doc = " signed durations."] # [derive (Clone , Copy , Debug)] pub (crate) enum SDuration { Span (Span) , Absolute (SignedDuration) , }
};
}
