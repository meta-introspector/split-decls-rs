// Generated macro for impl_127 (impl)
macro_rules! Depcrate_lazyimpl_127 {
() => {
// Module: crate::lazy
// Provides: {"impl_127"}
// Dependencies: {}
impl < F , R > Future for Lazy < F , R :: Future > where F : FnOnce () -> R + Send + 'static , R : IntoFuture , { type Item = R :: Item ; type Error = R :: Error ; fn poll (& mut self , task : & mut Task) -> Poll < R :: Item , R :: Error > { self . get () . poll (task) } fn schedule (& mut self , task : & mut Task) { self . get () . schedule (task) } fn tailcall (& mut self) -> Option < Box < Future < Item = R :: Item , Error = R :: Error > > > { self . get () . tailcall () } }
};
}
