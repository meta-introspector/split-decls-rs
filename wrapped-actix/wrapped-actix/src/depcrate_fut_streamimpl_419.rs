// Generated macro for impl_419 (impl)
macro_rules! Depcrate_fut_streamimpl_419 {
() => {
// Module: crate::fut::stream
// Provides: {"impl_419"}
// Dependencies: {}
impl < S , A > WrapStream < A > for S where S : Stream , A : Actor , { type Stream = StreamWrap < S , A > ; # [doc (hidden)] fn actstream (self) -> Self :: Stream { wrap_stream (self) } fn into_actor (self , _ : & A) -> Self :: Stream { wrap_stream (self) } }
};
}
