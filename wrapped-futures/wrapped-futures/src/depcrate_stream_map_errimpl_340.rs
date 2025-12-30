// Generated macro for impl_340 (impl)
macro_rules! Depcrate_stream_map_errimpl_340 {
() => {
// Module: crate::stream::map_err
// Provides: {"impl_340"}
// Dependencies: {}
impl < S , F , U > Stream for MapErr < S , F > where S : Stream , F : FnMut (S :: Error) -> U + Send + 'static , U : Send + 'static , { type Item = S :: Item ; type Error = U ; fn poll (& mut self , task : & mut Task) -> Poll < Option < S :: Item > , U > { self . stream . poll (task) . map_err (& mut self . f) } fn schedule (& mut self , task : & mut Task) { self . stream . schedule (task) } }
};
}
