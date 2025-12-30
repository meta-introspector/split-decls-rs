// Generated macro for impl_181 (impl)
macro_rules! Depcrate_registryimpl_181 {
() => {
// Module: crate::registry
// Provides: {"impl_181"}
// Dependencies: {}
impl ThreadInfo { fn new (stealer : Stealer < JobRef >) -> ThreadInfo { ThreadInfo { primed : LockLatch :: new () , stopped : LockLatch :: new () , terminate : OnceLatch :: new () , stealer , } } }
};
}
