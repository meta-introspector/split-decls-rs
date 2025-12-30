// Generated macro for impl_422 (impl)
macro_rules! Depcrate_sync_condvarimpl_422 {
() => {
// Module: crate::sync::condvar
// Provides: {"impl_422"}
// Dependencies: {}
impl Condvar { # [doc = " Creates a new condition variable which is ready to be waited on and notified."] pub fn new () -> Condvar { Condvar { object : rt :: Condvar :: new () , } } # [doc = " Blocks the current thread until this condition variable receives a notification."] # [track_caller] pub fn wait < 'a , T > (& self , mut guard : MutexGuard < 'a , T >) -> LockResult < MutexGuard < 'a , T > > { guard . unborrow () ; self . object . wait (guard . rt () , location ! ()) ; guard . reborrow () ; Ok (guard) } # [doc = " Waits on this condition variable for a notification, timing out after a"] # [doc = " specified duration."] pub fn wait_timeout < 'a , T > (& self , guard : MutexGuard < 'a , T > , _dur : Duration ,) -> LockResult < (MutexGuard < 'a , T > , WaitTimeoutResult) > { self . wait (guard) . map (| guard | (guard , WaitTimeoutResult (false))) . map_err (| err | PoisonError :: new ((err . into_inner () , WaitTimeoutResult (false)))) } # [doc = " Wakes up one blocked thread on this condvar."] # [track_caller] pub fn notify_one (& self) { self . object . notify_one (location ! ()) ; } # [doc = " Wakes up all blocked threads on this condvar."] # [track_caller] pub fn notify_all (& self) { self . object . notify_all (location ! ()) ; } }
};
}
