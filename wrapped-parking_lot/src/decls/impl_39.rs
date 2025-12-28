macro_rules! deps {
    () => {
        GuardMarker!();
        RawMutex!();
        RawFairMutex!();
    };
}

macro_rules! impl_39 {
    () => {
        deps!();
        unsafe impl lock_api :: RawMutex for RawFairMutex { const INIT : Self = RawFairMutex (< RawMutex as lock_api :: RawMutex > :: INIT) ; type GuardMarker = < RawMutex as lock_api :: RawMutex > :: GuardMarker ; # [inline] fn lock (& self) { self . 0 . lock () } # [inline] fn try_lock (& self) -> bool { self . 0 . try_lock () } # [inline] unsafe fn unlock (& self) { self . unlock_fair () } # [inline] fn is_locked (& self) -> bool { self . 0 . is_locked () } }
    };
}

impl_39!()