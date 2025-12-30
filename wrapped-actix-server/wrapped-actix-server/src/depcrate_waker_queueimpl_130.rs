// Generated macro for impl_130 (impl)
macro_rules! Depcrate_waker_queueimpl_130 {
() => {
// Module: crate::waker_queue
// Provides: {"impl_130"}
// Dependencies: {}
impl Deref for WakerQueue { type Target = (Waker , Mutex < VecDeque < WakerInterest > >) ; fn deref (& self) -> & Self :: Target { self . 0 . deref () } }
};
}
