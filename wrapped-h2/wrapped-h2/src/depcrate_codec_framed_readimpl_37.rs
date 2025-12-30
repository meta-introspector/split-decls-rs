// Generated macro for impl_37 (impl)
macro_rules! Depcrate_codec_framed_readimpl_37 {
() => {
// Module: crate::codec::framed_read
// Provides: {"impl_37"}
// Dependencies: {}
impl Continuable { fn stream_id (& self) -> frame :: StreamId { match * self { Continuable :: Headers (ref h) => h . stream_id () , Continuable :: PushPromise (ref p) => p . stream_id () , } } fn is_over_size (& self) -> bool { match * self { Continuable :: Headers (ref h) => h . is_over_size () , Continuable :: PushPromise (ref p) => p . is_over_size () , } } fn load_hpack (& mut self , src : & mut BytesMut , max_header_list_size : usize , decoder : & mut hpack :: Decoder ,) -> Result < () , frame :: Error > { match * self { Continuable :: Headers (ref mut h) => h . load_hpack (src , max_header_list_size , decoder) , Continuable :: PushPromise (ref mut p) => p . load_hpack (src , max_header_list_size , decoder) , } } }
};
}
