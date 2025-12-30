// Generated macro for impl_18 (impl)
macro_rules! Depcrate_scoredimpl_18 {
() => {
// Module: crate::scored
// Provides: {"impl_18"}
// Dependencies: {}
impl < K : PartialOrd , T > PartialOrd for MinScored < K , T > { # [inline] fn partial_cmp (& self , other : & MinScored < K , T >) -> Option < Ordering > { Some (self . cmp (other)) } }
};
}
