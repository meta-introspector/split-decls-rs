// Generated macro for impl_224 (impl)
macro_rules! Depcrate_thenimpl_224 {
() => {
// Module: crate::then
// Provides: {"impl_224"}
// Dependencies: {}
impl < A , B , F > Future for Then < A , B , F > where A : Future , B : IntoFuture , F : FnOnce (Result < A :: Item , A :: Error >) -> B + Send + 'static , { type Item = B :: Item ; type Error = B :: Error ; fn poll (& mut self , task : & mut Task) -> Poll < B :: Item , B :: Error > { self . state . poll (task , | a , f | { Ok (Err (f (a) . into_future ())) }) } fn schedule (& mut self , task : & mut Task) { self . state . schedule (task) } fn tailcall (& mut self) -> Option < Box < Future < Item = Self :: Item , Error = Self :: Error > > > { self . state . tailcall () } }
};
}
