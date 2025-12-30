// Generated macro for impl_59 (impl)
macro_rules! Depcrate_deimpl_59 {
() => {
// Module: crate::de
// Provides: {"impl_59"}
// Dependencies: {}
# [cfg (feature = "bytes")] impl BorshDeserialize for bytes :: Bytes { # [inline] fn deserialize_reader < R : Read > (reader : & mut R) -> Result < Self > { let vec = < Vec < u8 > > :: deserialize_reader (reader) ? ; Ok (vec . into ()) } }
};
}
