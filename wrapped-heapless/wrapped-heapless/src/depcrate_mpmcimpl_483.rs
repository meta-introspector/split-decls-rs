// Generated macro for impl_483 (impl)
macro_rules! Depcrate_mpmcimpl_483 {
() => {
// Module: crate::mpmc
// Provides: {"impl_483"}
// Dependencies: {}
impl < T , S : Storage > Drop for QueueInner < T , S > { fn drop (& mut self) { while self . dequeue () . is_some () { } } }
};
}
