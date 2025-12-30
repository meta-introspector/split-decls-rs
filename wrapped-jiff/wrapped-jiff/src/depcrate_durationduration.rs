// Generated macro for Duration (enum)
macro_rules! Depcrate_durationDuration {
() => {
// Module: crate::duration
// Provides: {"Duration"}
// Dependencies: {}
# [doc = " An internal type for abstracting over different duration types."] # [derive (Clone , Copy , Debug)] pub (crate) enum Duration { Span (Span) , Signed (SignedDuration) , Unsigned (UnsignedDuration) , }
};
}
