// Generated macro for impl_61 (impl)
macro_rules! Depcrate_core_ordered_queueimpl_61 {
() => {
// Module: crate::core::ordered_queue
// Provides: {"impl_61"}
// Dependencies: {}
impl < T > Clone for OrderedQueue < T > where T : Send , { fn clone (& self) -> Self { OrderedQueue { sender : self . sender . clone () , pending_count : self . pending_count . clone () , stop : self . stop . clone () , } } }
};
}
