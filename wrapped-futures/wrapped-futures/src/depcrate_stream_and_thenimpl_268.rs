// Generated macro for impl_268 (impl)
macro_rules! Depcrate_stream_and_thenimpl_268 {
() => {
// Module: crate::stream::and_then
// Provides: {"impl_268"}
// Dependencies: {}
impl < S , F , U > Stream for AndThen < S , F , U > where S : Stream , F : FnMut (S :: Item) -> U + Send + 'static , U : IntoFuture < Error = S :: Error > , { type Item = U :: Item ; type Error = S :: Error ; fn poll (& mut self , task : & mut Task) -> Poll < Option < U :: Item > , S :: Error > { if self . future . is_none () { let item = match try_poll ! (self . stream . poll (task)) { Ok (None) => return Poll :: Ok (None) , Ok (Some (e)) => e , Err (e) => return Poll :: Err (e) , } ; self . future = Some ((self . f) (item) . into_future ()) ; } assert ! (self . future . is_some ()) ; let res = self . future . as_mut () . unwrap () . poll (task) ; if res . is_ready () { self . future = None ; } res . map (Some) } fn schedule (& mut self , task : & mut Task) { match self . future { Some (ref mut s) => s . schedule (task) , None => self . stream . schedule (task) , } } }
};
}
