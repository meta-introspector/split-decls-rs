// Generated macro for impl_107 (impl)
macro_rules! Depcrate_stream_buf_readerimpl_107 {
() => {
// Module: crate::stream::buf_reader
// Provides: {"impl_107"}
// Dependencies: {}
# [cfg (feature = "futures-03")] impl < 'a , C , R > Future for ExtendBuf < 'a , C , R > where C : CombineAsyncRead < R > , { type Output = io :: Result < usize > ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let me = self . project () ; me . buffer . poll_extend_buf (cx , me . read . as_mut ()) } }
};
}
