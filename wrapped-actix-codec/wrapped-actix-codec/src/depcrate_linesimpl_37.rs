// Generated macro for impl_37 (impl)
macro_rules! Depcrate_linesimpl_37 {
() => {
// Module: crate::lines
// Provides: {"impl_37"}
// Dependencies: {}
impl < T : AsRef < str > > Encoder < T > for LinesCodec { type Error = io :: Error ; # [inline] fn encode (& mut self , item : T , dst : & mut BytesMut) -> Result < () , Self :: Error > { let item = item . as_ref () ; dst . reserve (item . len () + 1) ; dst . put_slice (item . as_bytes ()) ; dst . put_u8 (b'\n') ; Ok (()) } }
};
}
