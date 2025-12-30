// Generated macro for read_str_len_with_nread (function)
macro_rules! Depcrate_decode_strread_str_len_with_nread {
() => {
// Module: crate::decode::str
// Provides: {"read_str_len_with_nread"}
// Dependencies: {}
fn read_str_len_with_nread < R > (rd : & mut R) -> Result < (u32 , usize) , ValueReadError < R :: Error > > where R : RmpRead { match read_marker (rd) ? { Marker :: FixStr (size) => Ok ((u32 :: from (size) , 1)) , Marker :: Str8 => Ok ((u32 :: from (rd . read_data_u8 () ?) , 2)) , Marker :: Str16 => Ok ((u32 :: from (rd . read_data_u16 () ?) , 3)) , Marker :: Str32 => Ok ((rd . read_data_u32 () ? , 5)) , marker => Err (ValueReadError :: TypeMismatch (marker)) , } }
};
}
