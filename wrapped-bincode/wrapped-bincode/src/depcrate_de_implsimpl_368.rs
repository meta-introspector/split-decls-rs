// Generated macro for impl_368 (impl)
macro_rules! Depcrate_de_implsimpl_368 {
() => {
// Module: crate::de::impls
// Provides: {"impl_368"}
// Dependencies: {}
impl < Context > Decode < Context > for u16 { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { decoder . claim_bytes_read (2) ? ; match D :: C :: INT_ENCODING { IntEncoding :: Variable => { crate :: varint :: varint_decode_u16 (decoder . reader () , D :: C :: ENDIAN) } IntEncoding :: Fixed => { let mut bytes = [0u8 ; 2] ; decoder . reader () . read (& mut bytes) ? ; Ok (match D :: C :: ENDIAN { Endianness :: Little => u16 :: from_le_bytes (bytes) , Endianness :: Big => u16 :: from_be_bytes (bytes) , }) } } } }
};
}
