// Generated macro for impl_38 (impl)
macro_rules! Depcrate_codec_framed_readimpl_38 {
() => {
// Module: crate::codec::framed_read
// Provides: {"impl_38"}
// Dependencies: {}
impl < T > From < Continuable > for Frame < T > { fn from (cont : Continuable) -> Self { match cont { Continuable :: Headers (mut headers) => { headers . set_end_headers () ; headers . into () } Continuable :: PushPromise (mut push) => { push . set_end_headers () ; push . into () } } } }
};
}
