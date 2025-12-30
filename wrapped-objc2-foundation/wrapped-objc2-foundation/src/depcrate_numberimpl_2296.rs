// Generated macro for impl_2296 (impl)
macro_rules! Depcrate_numberimpl_2296 {
() => {
// Module: crate::number
// Provides: {"impl_2296"}
// Dependencies: {}
# [doc = " Beware: This uses the Objective-C method \"compare:\", which has different"] # [doc = " floating point NaN semantics than Rust!"] # [cfg (feature = "NSObjCRuntime")] impl Ord for NSNumber { # [doc (alias = "compare:")] fn cmp (& self , other : & Self) -> Ordering { self . compare (other) . into () } }
};
}
