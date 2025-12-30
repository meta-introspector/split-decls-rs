// Generated macro for impl_412 (impl)
macro_rules! Depcrate_de_implsimpl_412 {
() => {
// Module: crate::de::impls
// Provides: {"impl_412"}
// Dependencies: {}
impl < Context > Decode < Context > for f32 { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { decoder . claim_bytes_read (4) ? ; let mut bytes = [0u8 ; 4] ; decoder . reader () . read (& mut bytes) ? ; Ok (match D :: C :: ENDIAN { Endianness :: Little => f32 :: from_le_bytes (bytes) , Endianness :: Big => f32 :: from_be_bytes (bytes) , }) } }
};
}
