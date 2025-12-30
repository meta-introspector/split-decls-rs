// Generated macro for impl_328 (impl)
macro_rules! Depcrate_stream_futureimpl_328 {
() => {
// Module: crate::stream::future
// Provides: {"impl_328"}
// Dependencies: {}
impl < S : Stream > Future for StreamFuture < S > { type Item = (Option < S :: Item > , S) ; type Error = (S :: Error , S) ; fn poll (& mut self , task : & mut Task) -> Poll < Self :: Item , Self :: Error > { let item = { let s = self . stream . as_mut () . expect ("polling StreamFuture twice") ; try_poll ! (s . poll (task)) } ; let stream = self . stream . take () . unwrap () ; match item { Ok (e) => Poll :: Ok ((e , stream)) , Err (e) => Poll :: Err ((e , stream)) , } } fn schedule (& mut self , task : & mut Task) { if let Some (s) = self . stream . as_mut () { s . schedule (task) } } }
};
}
