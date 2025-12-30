// Generated macro for impl_162 (impl)
macro_rules! Depcrate_and_thenimpl_162 {
() => {
// Module: crate::and_then
// Provides: {"impl_162"}
// Dependencies: {}
impl < A , B , F > Future for AndThen < A , B , F > where A : Future , B : IntoFuture < Error = A :: Error > , F : FnOnce (A :: Item) -> B + Send + 'static , { type Item = B :: Item ; type Error = B :: Error ; fn poll (& mut self , task : & mut Task) -> Poll < B :: Item , B :: Error > { self . state . poll (task , | result , f | { result . map (| e | { Err (f (e) . into_future ()) }) }) } fn schedule (& mut self , task : & mut Task) { self . state . schedule (task) } fn tailcall (& mut self) -> Option < Box < Future < Item = Self :: Item , Error = Self :: Error > > > { self . state . tailcall () } }
};
}
