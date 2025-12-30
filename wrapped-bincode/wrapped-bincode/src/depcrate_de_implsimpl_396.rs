// Generated macro for impl_396 (impl)
macro_rules! Depcrate_de_implsimpl_396 {
() => {
// Module: crate::de::impls
// Provides: {"impl_396"}
// Dependencies: {}
impl < Context > Decode < Context > for i32 { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { decoder . claim_bytes_read (4) ? ; match D :: C :: INT_ENCODING { IntEncoding :: Variable => { crate :: varint :: varint_decode_i32 (decoder . reader () , D :: C :: ENDIAN) } IntEncoding :: Fixed => { let mut bytes = [0u8 ; 4] ; decoder . reader () . read (& mut bytes) ? ; Ok (match D :: C :: ENDIAN { Endianness :: Little => i32 :: from_le_bytes (bytes) , Endianness :: Big => i32 :: from_be_bytes (bytes) , }) } } } }
};
}
