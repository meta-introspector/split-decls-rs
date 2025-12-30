// Generated macro for impl_295 (impl)
macro_rules! Depcrate_stream_filter_mapimpl_295 {
() => {
// Module: crate::stream::filter_map
// Provides: {"impl_295"}
// Dependencies: {}
impl < S , F , B > Stream for FilterMap < S , F > where S : Stream , F : FnMut (S :: Item) -> Option < B > + Send + 'static , B : Send + 'static , { type Item = B ; type Error = S :: Error ; fn poll (& mut self , task : & mut Task) -> Poll < Option < B > , S :: Error > { loop { match try_poll ! (self . stream . poll (task)) { Ok (Some (e)) => { if let Some (e) = (self . f) (e) { return Poll :: Ok (Some (e)) } } Ok (None) => return Poll :: Ok (None) , Err (e) => return Poll :: Err (e) , } } } fn schedule (& mut self , task : & mut Task) { self . stream . schedule (task) } }
};
}
