// Generated macro for impl_9 (impl)
macro_rules! Depcrate_bcodecimpl_9 {
() => {
// Module: crate::bcodec
// Provides: {"impl_9"}
// Dependencies: {}
impl Decoder for BytesCodec { type Item = BytesMut ; type Error = io :: Error ; fn decode (& mut self , src : & mut BytesMut) -> Result < Option < Self :: Item > , Self :: Error > { if src . is_empty () { Ok (None) } else { Ok (Some (src . split ())) } } }
};
}
