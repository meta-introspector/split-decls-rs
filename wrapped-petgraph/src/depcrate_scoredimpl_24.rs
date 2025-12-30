// Generated macro for impl_24 (impl)
macro_rules! Depcrate_scoredimpl_24 {
() => {
// Module: crate::scored
// Provides: {"impl_24"}
// Dependencies: {}
impl < K : PartialOrd , T > Ord for MaxScored < K , T > { # [inline] fn cmp (& self , other : & MaxScored < K , T >) -> Ordering { let a = & self . 0 ; let b = & other . 0 ; if a == b { Ordering :: Equal } else if a < b { Ordering :: Less } else if a > b { Ordering :: Greater } else if a . ne (a) && b . ne (b) { Ordering :: Equal } else if a . ne (a) { Ordering :: Less } else { Ordering :: Greater } } }
};
}
