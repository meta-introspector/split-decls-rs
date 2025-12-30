// Generated macro for impl_15 (impl)
macro_rules! Depcrate_parking_lotimpl_15 {
() => {
// Module: crate::parking_lot
// Provides: {"impl_15"}
// Dependencies: {}
impl Bucket { # [inline] pub fn new (timeout : TimeoutInstant , seed : u32) -> Self { Self { mutex : WordLock :: new () , queue_head : Cell :: new (ptr :: null ()) , queue_tail : Cell :: new (ptr :: null ()) , fair_timeout : UnsafeCell :: new (FairTimeout :: new (timeout , seed)) , } } }
};
}
