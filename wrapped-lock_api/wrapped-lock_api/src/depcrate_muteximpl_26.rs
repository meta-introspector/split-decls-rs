// Generated macro for impl_26 (impl)
macro_rules! Depcrate_muteximpl_26 {
() => {
// Module: crate::mutex
// Provides: {"impl_26"}
// Dependencies: {}
impl < R : RawMutexFair , T : ? Sized > Mutex < R , T > { # [doc = " Forcibly unlocks the mutex using a fair unlock protocol."] # [doc = ""] # [doc = " This is useful when combined with `mem::forget` to hold a lock without"] # [doc = " the need to maintain a `MutexGuard` object alive, for example when"] # [doc = " dealing with FFI."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " This method must only be called if the current thread logically owns a"] # [doc = " `MutexGuard` but that guard has been discarded using `mem::forget`."] # [doc = " Behavior is undefined if a mutex is unlocked when not locked."] # [inline] # [track_caller] pub unsafe fn force_unlock_fair (& self) { self . raw . unlock_fair () ; } }
};
}
