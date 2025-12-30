// Generated macro for impl_527 (impl)
macro_rules! Depcrate_enc_implsimpl_527 {
() => {
// Module: crate::enc::impls
// Provides: {"impl_527"}
// Dependencies: {}
impl Encode for isize { fn encode < E : Encoder > (& self , encoder : & mut E) -> Result < () , EncodeError > { match E :: C :: INT_ENCODING { IntEncoding :: Variable => { crate :: varint :: varint_encode_isize (encoder . writer () , E :: C :: ENDIAN , * self) } IntEncoding :: Fixed => match E :: C :: ENDIAN { Endianness :: Big => encoder . writer () . write (& (* self as i64) . to_be_bytes ()) , Endianness :: Little => encoder . writer () . write (& (* self as i64) . to_le_bytes ()) , } , } } }
};
}
