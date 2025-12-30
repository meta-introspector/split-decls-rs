// Generated macro for impl_766 (impl)
macro_rules! Depcrate_unlock_notifyimpl_766 {
() => {
// Module: crate::unlock_notify
// Provides: {"impl_766"}
// Dependencies: {}
impl UnlockNotification { fn new () -> Self { Self { cond : Condvar :: new () , mutex : Mutex :: new (false) , } } fn fired (& self) { let mut flag = unpoison (self . mutex . lock ()) ; * flag = true ; self . cond . notify_one () ; } fn wait (& self) { let mut fired = unpoison (self . mutex . lock ()) ; while ! * fired { fired = unpoison (self . cond . wait (fired)) ; } } }
};
}
