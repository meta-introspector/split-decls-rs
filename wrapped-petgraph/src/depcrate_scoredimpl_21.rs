// Generated macro for impl_21 (impl)
macro_rules! Depcrate_scoredimpl_21 {
() => {
// Module: crate::scored
// Provides: {"impl_21"}
// Dependencies: {}
impl < K : PartialOrd , T > PartialEq for MaxScored < K , T > { # [inline] fn eq (& self , other : & MaxScored < K , T >) -> bool { self . cmp (other) == Ordering :: Equal } }
};
}
