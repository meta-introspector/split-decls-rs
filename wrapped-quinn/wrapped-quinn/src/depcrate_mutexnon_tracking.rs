// Generated macro for non_tracking (module)
macro_rules! Depcrate_mutexnon_tracking {
() => {
// Module: crate::mutex
// Provides: {"non_tracking"}
// Dependencies: {}
# [cfg (not (feature = "lock_tracking"))] mod non_tracking { use super :: * ; # [doc = " A Mutex which optionally allows to track the time a lock was held and"] # [doc = " emit warnings in case of excessive lock times"] # [derive (Debug)] pub (crate) struct Mutex < T > { inner : std :: sync :: Mutex < T > , } impl < T > Mutex < T > { pub (crate) fn new (value : T) -> Self { Self { inner : std :: sync :: Mutex :: new (value) , } } # [doc = " Acquires the lock for a certain purpose"] # [doc = ""] # [doc = " The purpose will be recorded in the list of last lock owners"] pub (crate) fn lock (& self , _purpose : & 'static str) -> MutexGuard < '_ , T > { MutexGuard { guard : self . inner . lock () . unwrap () , } } } pub (crate) struct MutexGuard < 'a , T > { guard : std :: sync :: MutexGuard < 'a , T > , } impl < T > Deref for MutexGuard < '_ , T > { type Target = T ; fn deref (& self) -> & Self :: Target { self . guard . deref () } } impl < T > DerefMut for MutexGuard < '_ , T > { fn deref_mut (& mut self) -> & mut Self :: Target { self . guard . deref_mut () } } }
};
}
