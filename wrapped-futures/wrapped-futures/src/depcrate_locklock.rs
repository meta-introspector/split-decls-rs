// Generated macro for Lock (struct)
macro_rules! Depcrate_lockLock {
() => {
// Module: crate::lock
// Provides: {"Lock"}
// Dependencies: {}
# [doc = " A \"mutex\" around a value, similar to `std::sync::Mutex<T>`."] # [doc = ""] # [doc = " This lock only supports the `try_lock` operation, however, and does not"] # [doc = " implement poisoning."] pub struct Lock < T > { locked : AtomicBool , data : UnsafeCell < T > , }
};
}
