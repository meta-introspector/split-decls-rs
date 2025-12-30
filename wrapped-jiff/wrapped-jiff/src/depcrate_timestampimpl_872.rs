// Generated macro for impl_872 (impl)
macro_rules! Depcrate_timestampimpl_872 {
() => {
// Module: crate::timestamp
// Provides: {"impl_872"}
// Dependencies: {}
impl PartialEq for Timestamp { # [inline] fn eq (& self , rhs : & Timestamp) -> bool { self . as_second_ranged () . get () == rhs . as_second_ranged () . get () && self . subsec_nanosecond_ranged () . get () == rhs . subsec_nanosecond_ranged () . get () } }
};
}
