// Generated macro for impl_8 (impl)
macro_rules! Depcrate_bcodecimpl_8 {
() => {
// Module: crate::bcodec
// Provides: {"impl_8"}
// Dependencies: {}
impl Encoder < Bytes > for BytesCodec { type Error = io :: Error ; # [inline] fn encode (& mut self , item : Bytes , dst : & mut BytesMut) -> Result < () , Self :: Error > { dst . extend_from_slice (item . chunk ()) ; Ok (()) } }
};
}
