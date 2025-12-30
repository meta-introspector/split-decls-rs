// Generated macro for impl_873 (impl)
macro_rules! Depcrate_timestampimpl_873 {
() => {
// Module: crate::timestamp
// Provides: {"impl_873"}
// Dependencies: {}
impl Ord for Timestamp { # [inline] fn cmp (& self , rhs : & Timestamp) -> core :: cmp :: Ordering { (self . as_second_ranged () . get () , self . subsec_nanosecond_ranged () . get ()) . cmp (& (rhs . as_second_ranged () . get () , rhs . subsec_nanosecond_ranged () . get () ,)) } }
};
}
