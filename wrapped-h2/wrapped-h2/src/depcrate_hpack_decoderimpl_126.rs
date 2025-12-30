// Generated macro for impl_126 (impl)
macro_rules! Depcrate_hpack_decoderimpl_126 {
() => {
// Module: crate::hpack::decoder
// Provides: {"impl_126"}
// Dependencies: {}
impl StringMarker { fn consume (self , buf : & mut Cursor < & mut BytesMut >) -> Bytes { buf . advance (self . offset) ; match self . string { Some (string) => { buf . advance (self . len) ; string } None => take (buf , self . len) , } } }
};
}
