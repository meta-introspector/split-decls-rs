// Generated macro for impl_44 (impl)
macro_rules! Depcrate_dequeimpl_44 {
() => {
// Module: crate::deque
// Provides: {"impl_44"}
// Dependencies: {}
impl < T > Slot < T > { # [doc = " Waits until a task is written into the slot."] fn wait_write (& self) { let backoff = Backoff :: new () ; while self . state . load (Ordering :: Acquire) & WRITE == 0 { backoff . snooze () ; } } }
};
}
