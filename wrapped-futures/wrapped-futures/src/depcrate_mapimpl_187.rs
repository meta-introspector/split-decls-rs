// Generated macro for impl_187 (impl)
macro_rules! Depcrate_mapimpl_187 {
() => {
// Module: crate::map
// Provides: {"impl_187"}
// Dependencies: {}
impl < U , A , F > Future for Map < A , F > where A : Future , F : FnOnce (A :: Item) -> U + Send + 'static , U : Send + 'static , { type Item = U ; type Error = A :: Error ; fn poll (& mut self , task : & mut Task) -> Poll < U , A :: Error > { let result = try_poll ! (self . future . poll (task)) ; result . map (self . f . take () . expect ("cannot poll Map twice")) . into () } fn schedule (& mut self , task : & mut Task) { self . future . schedule (task) } fn tailcall (& mut self) -> Option < Box < Future < Item = Self :: Item , Error = Self :: Error > > > { self . future . collapse () ; None } }
};
}
