// Generated macro for impl_37 (impl)
macro_rules! Depcrate_muteximpl_37 {
() => {
// Module: crate::mutex
// Provides: {"impl_37"}
// Dependencies: {}
impl < 'a , R : RawMutex + 'a , T : ? Sized + 'a > Deref for MutexGuard < 'a , R , T > { type Target = T ; # [inline] fn deref (& self) -> & T { unsafe { & * self . mutex . data . get () } } }
};
}
