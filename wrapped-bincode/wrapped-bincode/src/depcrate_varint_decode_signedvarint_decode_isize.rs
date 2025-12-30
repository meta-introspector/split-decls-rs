// Generated macro for varint_decode_isize (function)
macro_rules! Depcrate_varint_decode_signedvarint_decode_isize {
() => {
// Module: crate::varint::decode_signed
// Provides: {"varint_decode_isize"}
// Dependencies: {}
pub fn varint_decode_isize < R : Reader > (read : & mut R , endian : Endianness ,) -> Result < isize , DecodeError > { match varint_decode_i64 (read , endian) { Ok (val) => Ok (val as isize) , Err (DecodeError :: InvalidIntegerType { found , .. }) => { Err (DecodeError :: InvalidIntegerType { expected : IntegerType :: Isize , found : found . into_signed () , }) } Err (e) => Err (e) , } }
};
}
