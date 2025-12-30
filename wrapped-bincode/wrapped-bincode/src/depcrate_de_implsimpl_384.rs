// Generated macro for impl_384 (impl)
macro_rules! Depcrate_de_implsimpl_384 {
() => {
// Module: crate::de::impls
// Provides: {"impl_384"}
// Dependencies: {}
impl < Context > Decode < Context > for usize { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { decoder . claim_bytes_read (8) ? ; match D :: C :: INT_ENCODING { IntEncoding :: Variable => { crate :: varint :: varint_decode_usize (decoder . reader () , D :: C :: ENDIAN) } IntEncoding :: Fixed => { let mut bytes = [0u8 ; 8] ; decoder . reader () . read (& mut bytes) ? ; let value = match D :: C :: ENDIAN { Endianness :: Little => u64 :: from_le_bytes (bytes) , Endianness :: Big => u64 :: from_be_bytes (bytes) , } ; value . try_into () . map_err (| _ | DecodeError :: OutsideUsizeRange (value)) } } } }
};
}
