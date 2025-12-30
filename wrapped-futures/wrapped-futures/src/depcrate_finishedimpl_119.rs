// Generated macro for impl_119 (impl)
macro_rules! Depcrate_finishedimpl_119 {
() => {
// Module: crate::finished
// Provides: {"impl_119"}
// Dependencies: {}
impl < T , E > Future for Finished < T , E > where T : Send + 'static , E : Send + 'static , { type Item = T ; type Error = E ; fn poll (& mut self , _ : & mut Task) -> Poll < T , E > { Poll :: Ok (self . t . take () . expect ("cannot poll Finished twice")) } fn schedule (& mut self , task : & mut Task) { task . notify () ; } }
};
}
