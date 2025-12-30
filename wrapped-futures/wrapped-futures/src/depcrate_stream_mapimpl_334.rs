// Generated macro for impl_334 (impl)
macro_rules! Depcrate_stream_mapimpl_334 {
() => {
// Module: crate::stream::map
// Provides: {"impl_334"}
// Dependencies: {}
impl < S , F , U > Stream for Map < S , F > where S : Stream , F : FnMut (S :: Item) -> U + Send + 'static , U : Send + 'static , { type Item = U ; type Error = S :: Error ; fn poll (& mut self , task : & mut Task) -> Poll < Option < U > , S :: Error > { self . stream . poll (task) . map (| option | option . map (& mut self . f)) } fn schedule (& mut self , task : & mut Task) { self . stream . schedule (task) } }
};
}
