// Generated macro for impl_60 (impl)
macro_rules! Depcrate_deimpl_60 {
() => {
// Module: crate::de
// Provides: {"impl_60"}
// Dependencies: {}
# [cfg (feature = "bytes")] impl BorshDeserialize for bytes :: BytesMut { # [inline] fn deserialize_reader < R : Read > (reader : & mut R) -> Result < Self > { let len = u32 :: deserialize_reader (reader) ? ; let mut out = BytesMut :: with_capacity (hint :: cautious :: < u8 > (len)) ; for _ in 0 .. len { out . put_u8 (u8 :: deserialize_reader (reader) ?) ; } Ok (out) } }
};
}
