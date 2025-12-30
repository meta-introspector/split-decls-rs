// Generated macro for impl_113 (impl)
macro_rules! Depcrate_failedimpl_113 {
() => {
// Module: crate::failed
// Provides: {"impl_113"}
// Dependencies: {}
impl < T , E > Future for Failed < T , E > where T : Send + 'static , E : Send + 'static , { type Item = T ; type Error = E ; fn poll (& mut self , _ : & mut Task) -> Poll < T , E > { Poll :: Err (self . e . take () . expect ("cannot poll Failed twice")) } fn schedule (& mut self , task : & mut Task) { task . notify () ; } }
};
}
