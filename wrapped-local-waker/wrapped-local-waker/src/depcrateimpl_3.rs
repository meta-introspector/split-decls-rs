// Generated macro for impl_3 (impl)
macro_rules! Depcrateimpl_3 {
() => {
// Module: crate
// Provides: {"impl_3"}
// Dependencies: {}
impl LocalWaker { # [doc = " Creates a new, empty `LocalWaker`."] pub fn new () -> Self { LocalWaker :: default () } # [doc = " Registers the waker to be notified on calls to `wake`."] # [doc = ""] # [doc = " Returns `true` if waker was registered before."] # [inline] pub fn register (& self , waker : & Waker) -> bool { let last_waker = self . waker . replace (Some (waker . clone ())) ; last_waker . is_some () } # [doc = " Calls `wake` on the last `Waker` passed to `register`."] # [doc = ""] # [doc = " If `register` has not been called yet, then this does nothing."] # [inline] pub fn wake (& self) { if let Some (waker) = self . take () { waker . wake () ; } } # [doc = " Returns the last `Waker` passed to `register`, so that the user can wake it."] # [doc = ""] # [doc = " If a waker has not been registered, this returns `None`."] # [inline] pub fn take (& self) -> Option < Waker > { self . waker . take () } }
};
}
