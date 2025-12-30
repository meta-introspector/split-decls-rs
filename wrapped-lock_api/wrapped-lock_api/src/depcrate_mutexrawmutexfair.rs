// Generated macro for RawMutexFair (trait)
macro_rules! Depcrate_mutexRawMutexFair {
() => {
// Module: crate::mutex
// Provides: {"RawMutexFair"}
// Dependencies: {}
# [doc = " Additional methods for mutexes which support fair unlocking."] # [doc = ""] # [doc = " Fair unlocking means that a lock is handed directly over to the next waiting"] # [doc = " thread if there is one, without giving other threads the opportunity to"] # [doc = " \"steal\" the lock in the meantime. This is typically slower than unfair"] # [doc = " unlocking, but may be necessary in certain circumstances."] pub unsafe trait RawMutexFair : RawMutex { # [doc = " Unlocks this mutex using a fair unlock protocol."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " This method may only be called if the mutex is held in the current context, see"] # [doc = " the documentation of [`unlock`](RawMutex::unlock)."] unsafe fn unlock_fair (& self) ; # [doc = " Temporarily yields the mutex to a waiting thread if there is one."] # [doc = ""] # [doc = " This method is functionally equivalent to calling `unlock_fair` followed"] # [doc = " by `lock`, however it can be much more efficient in the case where there"] # [doc = " are no waiting threads."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " This method may only be called if the mutex is held in the current context, see"] # [doc = " the documentation of [`unlock`](RawMutex::unlock)."] unsafe fn bump (& self) { self . unlock_fair () ; self . lock () ; } }
};
}
