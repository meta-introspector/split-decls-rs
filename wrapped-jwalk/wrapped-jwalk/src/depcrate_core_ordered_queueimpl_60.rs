// Generated macro for impl_60 (impl)
macro_rules! Depcrate_core_ordered_queueimpl_60 {
() => {
// Module: crate::core::ordered_queue
// Provides: {"impl_60"}
// Dependencies: {}
impl < T > OrderedQueue < T > where T : Send , { pub fn push (& self , ordered : Ordered < T >) -> Result < () , SendError < Ordered < T > > > { self . pending_count . fetch_add (1 , AtomicOrdering :: SeqCst) ; self . sender . send (ordered) } pub fn complete_item (& self) { self . pending_count . fetch_sub (1 , AtomicOrdering :: SeqCst) ; } }
};
}
