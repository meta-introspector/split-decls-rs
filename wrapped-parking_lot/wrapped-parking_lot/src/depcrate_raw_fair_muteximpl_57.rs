// Generated macro for impl_57 (impl)
macro_rules! Depcrate_raw_fair_muteximpl_57 {
() => {
// Module: crate::raw_fair_mutex
// Provides: {"impl_57"}
// Dependencies: {}
unsafe impl lock_api :: RawMutex for RawFairMutex { const INIT : Self = RawFairMutex (< RawMutex as lock_api :: RawMutex > :: INIT) ; type GuardMarker = < RawMutex as lock_api :: RawMutex > :: GuardMarker ; # [inline] fn lock (& self) { self . 0 . lock () } # [inline] fn try_lock (& self) -> bool { self . 0 . try_lock () } # [inline] unsafe fn unlock (& self) { self . unlock_fair () } # [inline] fn is_locked (& self) -> bool { self . 0 . is_locked () } }
};
}
