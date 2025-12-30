// Generated macro for impl_199 (impl)
macro_rules! Depcrate_or_elseimpl_199 {
() => {
// Module: crate::or_else
// Provides: {"impl_199"}
// Dependencies: {}
impl < A , B , F > Future for OrElse < A , B , F > where A : Future , B : IntoFuture < Item = A :: Item > , F : FnOnce (A :: Error) -> B + Send + 'static , { type Item = B :: Item ; type Error = B :: Error ; fn poll (& mut self , task : & mut Task) -> Poll < B :: Item , B :: Error > { self . state . poll (task , | a , f | { match a { Ok (item) => Ok (Ok (item)) , Err (e) => Ok (Err (f (e) . into_future ())) } }) } fn schedule (& mut self , task : & mut Task) { self . state . schedule (task) } fn tailcall (& mut self) -> Option < Box < Future < Item = Self :: Item , Error = Self :: Error > > > { self . state . tailcall () } }
};
}
