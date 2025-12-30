// Generated macro for read_str_data (function)
macro_rules! Depcrate_decode_strread_str_data {
() => {
// Module: crate::decode::str
// Provides: {"read_str_data"}
// Dependencies: {}
pub fn read_str_data < 'r , R > (rd : & mut R , len : u32 , buf : & 'r mut [u8]) -> Result < & 'r str , DecodeStringError < 'r , R :: Error > > where R : RmpRead { debug_assert_eq ! (len as usize , buf . len ()) ; match rd . read_exact_buf (buf) { Ok (()) => match from_utf8 (buf) { Ok (decoded) => Ok (decoded) , Err (err) => Err (DecodeStringError :: InvalidUtf8 (buf , err)) , } , Err (err) => Err (DecodeStringError :: InvalidDataRead (err)) , } }
};
}
