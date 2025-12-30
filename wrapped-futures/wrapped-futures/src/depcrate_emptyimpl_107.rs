// Generated macro for impl_107 (impl)
macro_rules! Depcrate_emptyimpl_107 {
() => {
// Module: crate::empty
// Provides: {"impl_107"}
// Dependencies: {}
impl < T , E > Future for Empty < T , E > where T : Send + 'static , E : Send + 'static , { type Item = T ; type Error = E ; fn poll (& mut self , _ : & mut Task) -> Poll < T , E > { Poll :: NotReady } fn schedule (& mut self , task : & mut Task) { drop (task) ; } }
};
}
