// Generated macro for impl_1234 (impl)
macro_rules! Depcrate_timeimpl_1234 {
() => {
// Module: crate::time
// Provides: {"impl_1234"}
// Dependencies: {}
impl Ord for Time { fn cmp (& self , other : & Time) -> Ordering { (self . raw . time , self . raw . offset) . cmp (& (other . raw . time , other . raw . offset)) } }
};
}
