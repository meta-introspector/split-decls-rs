// Generated macro for impl_131 (impl)
macro_rules! Depcrate_waker_queueimpl_131 {
() => {
// Module: crate::waker_queue
// Provides: {"impl_131"}
// Dependencies: {}
impl WakerQueue { # [doc = " Construct a waker queue with given `Poll`'s `Registry` and capacity."] # [doc = ""] # [doc = " A fixed `WAKER_TOKEN` is used to identify the wake interest and the `Poll` needs to match"] # [doc = " event's token for it to properly handle `WakerInterest`."] pub (crate) fn new (registry : & Registry) -> std :: io :: Result < Self > { let waker = Waker :: new (registry , WAKER_TOKEN) ? ; let queue = Mutex :: new (VecDeque :: with_capacity (16)) ; Ok (Self (Arc :: new ((waker , queue)))) } # [doc = " Push a new interest to the queue and wake up the accept poll afterwards."] pub (crate) fn wake (& self , interest : WakerInterest) { let (waker , queue) = self . deref () ; queue . lock () . expect ("Failed to lock WakerQueue") . push_back (interest) ; waker . wake () . unwrap_or_else (| err | panic ! ("can not wake up Accept Poll: {err}")) ; } # [doc = " Get a MutexGuard of the waker queue."] pub (crate) fn guard (& self) -> MutexGuard < '_ , VecDeque < WakerInterest > > { self . deref () . 1 . lock () . expect ("Failed to lock WakerQueue") } # [doc = " Reset the waker queue so it does not grow infinitely."] pub (crate) fn reset (queue : & mut VecDeque < WakerInterest >) { * queue = VecDeque :: < WakerInterest > :: with_capacity (16) ; } }
};
}
