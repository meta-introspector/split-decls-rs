// Generated macro for impl_23 (impl)
macro_rules! Depcrate_scoredimpl_23 {
() => {
// Module: crate::scored
// Provides: {"impl_23"}
// Dependencies: {}
impl < K : PartialOrd , T > PartialOrd for MaxScored < K , T > { # [inline] fn partial_cmp (& self , other : & MaxScored < K , T >) -> Option < Ordering > { Some (self . cmp (other)) } }
};
}
