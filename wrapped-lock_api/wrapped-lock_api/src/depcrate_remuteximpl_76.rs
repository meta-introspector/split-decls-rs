// Generated macro for impl_76 (impl)
macro_rules! Depcrate_remuteximpl_76 {
() => {
// Module: crate::remutex
// Provides: {"impl_76"}
// Dependencies: {}
impl < R : RawMutexFair , G : GetThreadId > RawReentrantMutex < R , G > { # [doc = " Unlocks this mutex using a fair unlock protocol. The inner mutex"] # [doc = " may not be unlocked if this mutex was acquired previously in the"] # [doc = " current thread."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " This method may only be called if the mutex is held by the current thread."] # [inline] pub unsafe fn unlock_fair (& self) { let lock_count = self . lock_count . get () - 1 ; self . lock_count . set (lock_count) ; if lock_count == 0 { self . owner . store (0 , Ordering :: Relaxed) ; self . mutex . unlock_fair () ; } } # [doc = " Temporarily yields the mutex to a waiting thread if there is one."] # [doc = ""] # [doc = " This method is functionally equivalent to calling `unlock_fair` followed"] # [doc = " by `lock`, however it can be much more efficient in the case where there"] # [doc = " are no waiting threads."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " This method may only be called if the mutex is held by the current thread."] # [inline] pub unsafe fn bump (& self) { if self . lock_count . get () == 1 { let id = self . owner . load (Ordering :: Relaxed) ; self . owner . store (0 , Ordering :: Relaxed) ; self . lock_count . set (0) ; self . mutex . bump () ; self . owner . store (id , Ordering :: Relaxed) ; self . lock_count . set (1) ; } } }
};
}
