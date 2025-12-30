// Generated macro for impl_366 (impl)
macro_rules! Depcrate_stream_skip_whileimpl_366 {
() => {
// Module: crate::stream::skip_while
// Provides: {"impl_366"}
// Dependencies: {}
impl < S , P , R > SkipWhile < S , P , R > where S : Stream , P : FnMut (& S :: Item) -> R + Send + 'static , R : IntoFuture < Item = bool , Error = S :: Error > , { # [doc = " Consume this adaptor, returning the underlying stream."] # [doc = ""] # [doc = " Note that if an element is buffered or a future is active determining"] # [doc = " whether that element should be yielded they will both be dropped as part"] # [doc = " of this operation."] pub fn into_inner (self) -> S { self . stream } }
};
}
