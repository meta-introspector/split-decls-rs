// Generated macro for impl_321 (impl)
macro_rules! Depcrate_stream_fuseimpl_321 {
() => {
// Module: crate::stream::fuse
// Provides: {"impl_321"}
// Dependencies: {}
impl < S : Stream > Stream for Fuse < S > { type Item = S :: Item ; type Error = S :: Error ; fn poll (& mut self , task : & mut Task) -> Poll < Option < S :: Item > , S :: Error > { let ret = self . stream . as_mut () . map (| s | s . poll (task)) ; match ret { Some (Poll :: Ok (None)) => self . stream = None , _ => { } } ret . unwrap_or (Poll :: NotReady) } fn schedule (& mut self , task : & mut Task) { if let Some (ref mut stream) = self . stream { stream . schedule (task) } } }
};
}
