// Generated macro for impl_414 (impl)
macro_rules! Depcrate_de_implsimpl_414 {
() => {
// Module: crate::de::impls
// Provides: {"impl_414"}
// Dependencies: {}
impl < Context > Decode < Context > for f64 { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { decoder . claim_bytes_read (8) ? ; let mut bytes = [0u8 ; 8] ; decoder . reader () . read (& mut bytes) ? ; Ok (match D :: C :: ENDIAN { Endianness :: Little => f64 :: from_le_bytes (bytes) , Endianness :: Big => f64 :: from_be_bytes (bytes) , }) } }
};
}
