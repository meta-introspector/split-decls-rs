// Generated macro for impl_353 (impl)
macro_rules! Depcrate_stream_or_elseimpl_353 {
() => {
// Module: crate::stream::or_else
// Provides: {"impl_353"}
// Dependencies: {}
impl < S , F , U > Stream for OrElse < S , F , U > where S : Stream , F : FnMut (S :: Error) -> U + Send + 'static , U : IntoFuture < Item = S :: Item > , { type Item = S :: Item ; type Error = U :: Error ; fn poll (& mut self , task : & mut Task) -> Poll < Option < S :: Item > , U :: Error > { if self . future . is_none () { let item = match try_poll ! (self . stream . poll (task)) { Ok (e) => return Poll :: Ok (e) , Err (e) => e , } ; self . future = Some ((self . f) (item) . into_future ()) ; } assert ! (self . future . is_some ()) ; let res = self . future . as_mut () . unwrap () . poll (task) ; if res . is_ready () { self . future = None ; } res . map (Some) } fn schedule (& mut self , task : & mut Task) { match self . future { Some (ref mut s) => s . schedule (task) , None => self . stream . schedule (task) , } } }
};
}
