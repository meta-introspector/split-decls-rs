// Generated macro for impl_359 (impl)
macro_rules! Depcrate_stream_skipimpl_359 {
() => {
// Module: crate::stream::skip
// Provides: {"impl_359"}
// Dependencies: {}
impl < S > Stream for Skip < S > where S : Stream , { type Item = S :: Item ; type Error = S :: Error ; fn poll (& mut self , task : & mut Task) -> Poll < Option < S :: Item > , S :: Error > { while self . remaining > 0 { match try_poll ! (self . stream . poll (task)) { Ok (Some (_)) => self . remaining -= 1 , Ok (None) => return Poll :: Ok (None) , Err (e) => return Poll :: Err (e) , } } self . stream . poll (task) } fn schedule (& mut self , task : & mut Task) { self . stream . schedule (task) } }
};
}
