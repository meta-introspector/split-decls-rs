// Generated macro for impl_2295 (impl)
macro_rules! Depcrate_numberimpl_2295 {
() => {
// Module: crate::number
// Provides: {"impl_2295"}
// Dependencies: {}
# [doc = " Beware: This uses the Objective-C method \"compare:\", which has different"] # [doc = " floating point NaN semantics than Rust!"] # [cfg (feature = "NSObjCRuntime")] impl PartialOrd for NSNumber { # [doc (alias = "compare:")] fn partial_cmp (& self , other : & Self) -> Option < Ordering > { Some (self . cmp (other)) } }
};
}
