// Generated macro for impl_56 (impl)
macro_rules! Depcrate_muteximpl_56 {
() => {
// Module: crate::mutex
// Provides: {"impl_56"}
// Dependencies: {}
impl < 'a , R : RawMutex + 'a , T : ? Sized + 'a > Deref for MappedMutexGuard < 'a , R , T > { type Target = T ; # [inline] fn deref (& self) -> & T { unsafe { & * self . data } } }
};
}
