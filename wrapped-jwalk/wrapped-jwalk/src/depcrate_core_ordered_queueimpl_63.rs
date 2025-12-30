// Generated macro for impl_63 (impl)
macro_rules! Depcrate_core_ordered_queueimpl_63 {
() => {
// Module: crate::core::ordered_queue
// Provides: {"impl_63"}
// Dependencies: {}
impl < T > Iterator for OrderedQueueIter < T > where T : Send , { type Item = Ordered < T > ; fn next (& mut self) -> Option < Ordered < T > > { loop { let try_next = match self . ordering { Ordering :: Relaxed => self . try_next_relaxed () , Ordering :: Strict => self . try_next_strict () , } ; match try_next { Ok (next) => { return Some (next) ; } Err (err) => match err { TryRecvError :: Empty => thread :: yield_now () , TryRecvError :: Disconnected => return None , } , } } } }
};
}
