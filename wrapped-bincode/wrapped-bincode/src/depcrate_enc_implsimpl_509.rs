// Generated macro for impl_509 (impl)
macro_rules! Depcrate_enc_implsimpl_509 {
() => {
// Module: crate::enc::impls
// Provides: {"impl_509"}
// Dependencies: {}
impl Encode for u32 { fn encode < E : Encoder > (& self , encoder : & mut E) -> Result < () , EncodeError > { match E :: C :: INT_ENCODING { IntEncoding :: Variable => { crate :: varint :: varint_encode_u32 (encoder . writer () , E :: C :: ENDIAN , * self) } IntEncoding :: Fixed => match E :: C :: ENDIAN { Endianness :: Big => encoder . writer () . write (& self . to_be_bytes ()) , Endianness :: Little => encoder . writer () . write (& self . to_le_bytes ()) , } , } } }
};
}
