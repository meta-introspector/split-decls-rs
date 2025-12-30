// Generated macro for impl_372 (impl)
macro_rules! Depcrate_stream_takeimpl_372 {
() => {
// Module: crate::stream::take
// Provides: {"impl_372"}
// Dependencies: {}
impl < S > Stream for Take < S > where S : Stream , { type Item = S :: Item ; type Error = S :: Error ; fn poll (& mut self , task : & mut Task) -> Poll < Option < S :: Item > , S :: Error > { if self . remaining == 0 { Poll :: Ok (None) } else { match self . stream . poll (task) { Poll :: Ok (Some (e)) => { self . remaining -= 1 ; Poll :: Ok (Some (e)) } other => other , } } } fn schedule (& mut self , task : & mut Task) { if self . remaining == 0 { task . notify () } else { self . stream . schedule (task) } } }
};
}
