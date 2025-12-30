// Generated macro for impl_31 (impl)
macro_rules! Depcrate_notifyimpl_31 {
() => {
// Module: crate::notify
// Provides: {"impl_31"}
// Dependencies: {}
impl < T , F : FnMut () -> T > TagProducer for F { type Tag = T ; fn next_tag (& mut self) -> T { (self) () } }
};
}
