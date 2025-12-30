// Generated macro for impl_529 (impl)
macro_rules! Depcrate_enc_implsimpl_529 {
() => {
// Module: crate::enc::impls
// Provides: {"impl_529"}
// Dependencies: {}
impl Encode for f32 { fn encode < E : Encoder > (& self , encoder : & mut E) -> Result < () , EncodeError > { match E :: C :: ENDIAN { Endianness :: Big => encoder . writer () . write (& self . to_be_bytes ()) , Endianness :: Little => encoder . writer () . write (& self . to_le_bytes ()) , } } }
};
}
