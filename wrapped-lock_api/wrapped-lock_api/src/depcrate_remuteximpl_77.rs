// Generated macro for impl_77 (impl)
macro_rules! Depcrate_remuteximpl_77 {
() => {
// Module: crate::remutex
// Provides: {"impl_77"}
// Dependencies: {}
impl < R : RawMutexTimed , G : GetThreadId > RawReentrantMutex < R , G > { # [doc = " Attempts to acquire this lock until a timeout is reached."] # [inline] pub fn try_lock_until (& self , timeout : R :: Instant) -> bool { self . lock_internal (| | self . mutex . try_lock_until (timeout)) } # [doc = " Attempts to acquire this lock until a timeout is reached."] # [inline] pub fn try_lock_for (& self , timeout : R :: Duration) -> bool { self . lock_internal (| | self . mutex . try_lock_for (timeout)) } }
};
}
