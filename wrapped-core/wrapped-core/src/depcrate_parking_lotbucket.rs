// Generated macro for Bucket (struct)
macro_rules! Depcrate_parking_lotBucket {
() => {
// Module: crate::parking_lot
// Provides: {"Bucket"}
// Dependencies: {}
# [repr (align (64))] struct Bucket { mutex : WordLock , queue_head : Cell < * const ThreadData > , queue_tail : Cell < * const ThreadData > , fair_timeout : UnsafeCell < FairTimeout > , }
};
}
