// Generated macro for impl_289 (impl)
macro_rules! Depcrate_stream_filterimpl_289 {
() => {
// Module: crate::stream::filter
// Provides: {"impl_289"}
// Dependencies: {}
impl < S , F > Stream for Filter < S , F > where S : Stream , F : FnMut (& S :: Item) -> bool + Send + 'static , { type Item = S :: Item ; type Error = S :: Error ; fn poll (& mut self , task : & mut Task) -> Poll < Option < S :: Item > , S :: Error > { loop { match try_poll ! (self . stream . poll (task)) { Ok (Some (e)) => { if (self . f) (& e) { return Poll :: Ok (Some (e)) } } Ok (None) => return Poll :: Ok (None) , Err (e) => return Poll :: Err (e) , } } } fn schedule (& mut self , task : & mut Task) { self . stream . schedule (task) } }
};
}
