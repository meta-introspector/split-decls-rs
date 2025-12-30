// Generated macro for impl_101 (impl)
macro_rules! Depcrate_doneimpl_101 {
() => {
// Module: crate::done
// Provides: {"impl_101"}
// Dependencies: {}
impl < T , E > Future for Done < T , E > where T : Send + 'static , E : Send + 'static , { type Item = T ; type Error = E ; fn poll (& mut self , _task : & mut Task) -> Poll < T , E > { self . inner . take () . expect ("cannot poll Done twice") . into () } fn schedule (& mut self , task : & mut Task) { task . notify () ; } }
};
}
