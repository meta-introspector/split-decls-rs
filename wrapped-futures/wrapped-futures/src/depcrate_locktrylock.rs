// Generated macro for TryLock (struct)
macro_rules! Depcrate_lockTryLock {
() => {
// Module: crate::lock
// Provides: {"TryLock"}
// Dependencies: {}
# [doc = " Sentinel representing an acquired lock through which the data can be"] # [doc = " accessed."] pub struct TryLock < 'a , T : 'a > { __ptr : & 'a Lock < T > , }
};
}
