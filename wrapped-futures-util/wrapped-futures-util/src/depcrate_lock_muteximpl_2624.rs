// Generated macro for impl_2624 (impl)
macro_rules! Depcrate_lock_muteximpl_2624 {
() => {
// Module: crate::lock::mutex
// Provides: {"impl_2624"}
// Dependencies: {}
impl Waiter { fn register (& mut self , waker : & Waker) { match self { Self :: Waiting (w) if waker . will_wake (w) => { } _ => * self = Self :: Waiting (waker . clone ()) , } } fn wake (& mut self) { match mem :: replace (self , Self :: Woken) { Self :: Waiting (waker) => waker . wake () , Self :: Woken => { } } } }
};
}
