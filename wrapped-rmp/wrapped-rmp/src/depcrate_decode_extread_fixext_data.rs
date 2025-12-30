// Generated macro for read_fixext_data (function)
macro_rules! Depcrate_decode_extread_fixext_data {
() => {
// Module: crate::decode::ext
// Provides: {"read_fixext_data"}
// Dependencies: {}
fn read_fixext_data < R : RmpRead > (rd : & mut R , buf : & mut [u8]) -> Result < i8 , ValueReadError < R :: Error > > { let id = rd . read_data_i8 () ? ; match rd . read_exact_buf (buf) { Ok (()) => Ok (id) , Err (err) => Err (ValueReadError :: InvalidDataRead (err)) , } }
};
}
