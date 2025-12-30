// Generated macro for impl_59 (impl)
macro_rules! Depcrate_raw_fair_muteximpl_59 {
() => {
// Module: crate::raw_fair_mutex
// Provides: {"impl_59"}
// Dependencies: {}
unsafe impl lock_api :: RawMutexTimed for RawFairMutex { type Duration = < RawMutex as lock_api :: RawMutexTimed > :: Duration ; type Instant = < RawMutex as lock_api :: RawMutexTimed > :: Instant ; # [inline] fn try_lock_until (& self , timeout : Self :: Instant) -> bool { self . 0 . try_lock_until (timeout) } # [inline] fn try_lock_for (& self , timeout : Self :: Duration) -> bool { self . 0 . try_lock_for (timeout) } }
};
}
