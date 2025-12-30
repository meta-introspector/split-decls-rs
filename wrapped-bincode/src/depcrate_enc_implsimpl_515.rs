// Generated macro for impl_515 (impl)
macro_rules! Depcrate_enc_implsimpl_515 {
() => {
// Module: crate::enc::impls
// Provides: {"impl_515"}
// Dependencies: {}
impl Encode for usize { fn encode < E : Encoder > (& self , encoder : & mut E) -> Result < () , EncodeError > { match E :: C :: INT_ENCODING { IntEncoding :: Variable => { crate :: varint :: varint_encode_usize (encoder . writer () , E :: C :: ENDIAN , * self) } IntEncoding :: Fixed => match E :: C :: ENDIAN { Endianness :: Big => encoder . writer () . write (& (* self as u64) . to_be_bytes ()) , Endianness :: Little => encoder . writer () . write (& (* self as u64) . to_le_bytes ()) , } , } } }
};
}
