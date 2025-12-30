// Generated macro for impl_42 (impl)
macro_rules! Depcrate_seg_queueimpl_42 {
() => {
// Module: crate::seg_queue
// Provides: {"impl_42"}
// Dependencies: {}
impl < T > Slot < T > { # [doc = " Waits until a value is written into the slot."] fn wait_write (& self) { let backoff = Backoff :: new () ; while self . state . load (Ordering :: Acquire) & WRITE == 0 { backoff . snooze () ; } } }
};
}
