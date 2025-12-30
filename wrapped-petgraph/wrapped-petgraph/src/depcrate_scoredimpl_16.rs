// Generated macro for impl_16 (impl)
macro_rules! Depcrate_scoredimpl_16 {
() => {
// Module: crate::scored
// Provides: {"impl_16"}
// Dependencies: {}
impl < K : PartialOrd , T > PartialEq for MinScored < K , T > { # [inline] fn eq (& self , other : & MinScored < K , T >) -> bool { self . cmp (other) == Ordering :: Equal } }
};
}
