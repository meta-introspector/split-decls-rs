// Generated macro for impl_994 (impl)
macro_rules! Depcrateimpl_994 {
() => {
// Module: crate
// Provides: {"impl_994"}
// Dependencies: {}
impl coding :: Codec for StreamId { fn decode < B : bytes :: Buf > (buf : & mut B) -> coding :: Result < Self > { VarInt :: decode (buf) . map (| x | Self (x . into_inner ())) } fn encode < B : bytes :: BufMut > (& self , buf : & mut B) { VarInt :: from_u64 (self . 0) . unwrap () . encode (buf) ; } }
};
}
