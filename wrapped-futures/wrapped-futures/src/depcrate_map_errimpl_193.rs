// Generated macro for impl_193 (impl)
macro_rules! Depcrate_map_errimpl_193 {
() => {
// Module: crate::map_err
// Provides: {"impl_193"}
// Dependencies: {}
impl < U , A , F > Future for MapErr < A , F > where A : Future , F : FnOnce (A :: Error) -> U + Send + 'static , U : Send + 'static , { type Item = A :: Item ; type Error = U ; fn poll (& mut self , task : & mut Task) -> Poll < A :: Item , U > { let result = try_poll ! (self . future . poll (task)) ; result . map_err (self . f . take () . expect ("cannot poll MapErr twice")) . into () } fn schedule (& mut self , task : & mut Task) { self . future . schedule (task) } fn tailcall (& mut self) -> Option < Box < Future < Item = Self :: Item , Error = Self :: Error > > > { self . future . collapse () ; None } }
};
}
