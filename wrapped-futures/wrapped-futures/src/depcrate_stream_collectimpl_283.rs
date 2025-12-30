// Generated macro for impl_283 (impl)
macro_rules! Depcrate_stream_collectimpl_283 {
() => {
// Module: crate::stream::collect
// Provides: {"impl_283"}
// Dependencies: {}
impl < S > Future for Collect < S > where S : Stream , { type Item = Vec < S :: Item > ; type Error = S :: Error ; fn poll (& mut self , task : & mut Task) -> Poll < Vec < S :: Item > , S :: Error > { loop { match try_poll ! (self . stream . poll (task)) { Ok (Some (e)) => self . items . push (e) , Ok (None) => return Poll :: Ok (self . finish ()) , Err (e) => { self . finish () ; return Poll :: Err (e) } } } } fn schedule (& mut self , task : & mut Task) { self . stream . schedule (task) } }
};
}
