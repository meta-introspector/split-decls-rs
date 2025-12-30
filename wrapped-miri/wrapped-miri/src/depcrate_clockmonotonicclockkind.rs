// Generated macro for MonotonicClockKind (enum)
macro_rules! Depcrate_clockMonotonicClockKind {
() => {
// Module: crate::clock
// Provides: {"MonotonicClockKind"}
// Dependencies: {}
# [derive (Debug)] enum MonotonicClockKind { Host { # [doc = " The \"epoch\" for this machine's monotone clock:"] # [doc = " the moment we consider to be time = 0."] epoch : StdInstant , } , Virtual { # [doc = " The \"current virtual time\"."] nanoseconds : Cell < u128 > , } , }
};
}
