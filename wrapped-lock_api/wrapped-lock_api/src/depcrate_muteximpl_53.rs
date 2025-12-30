// Generated macro for impl_53 (impl)
macro_rules! Depcrate_muteximpl_53 {
() => {
// Module: crate::mutex
// Provides: {"impl_53"}
// Dependencies: {}
unsafe impl < 'a , R : RawMutex + 'a , T : ? Sized + Send + 'a > Send for MappedMutexGuard < 'a , R , T > where R :: GuardMarker : Send { }
};
}
