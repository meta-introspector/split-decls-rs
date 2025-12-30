// Generated macro for AtomicPtr (struct)
macro_rules! Depcrate_sync_atomic_ptrAtomicPtr {
() => {
// Module: crate::sync::atomic::ptr
// Provides: {"AtomicPtr"}
// Dependencies: {}
# [doc = " Mock implementation of `std::sync::atomic::AtomicPtr`."] # [doc = ""] # [doc = " NOTE: Unlike `std::sync::atomic::AtomicPtr`, this type has a different"] # [doc = " in-memory representation than `*mut T`."] pub struct AtomicPtr < T > (Atomic < * mut T >) ;
};
}
