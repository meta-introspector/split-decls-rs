// Generated macro for impl_183 (impl)
macro_rules! Depcrate_flavors_listimpl_183 {
() => {
// Module: crate::flavors::list
// Provides: {"impl_183"}
// Dependencies: {}
impl < T > Slot < T > { # [doc = " Waits until a message is written into the slot."] fn wait_write (& self) { let backoff = Backoff :: new () ; while self . state . load (Ordering :: Acquire) & WRITE == 0 { backoff . snooze () ; } } }
};
}
