// Generated macro for read_array_len (function)
macro_rules! Depcrate_decoderead_array_len {
() => {
// Module: crate::decode
// Provides: {"read_array_len"}
// Dependencies: {}
# [doc = " Attempts to read up to 5 bytes from the given reader and to decode them as a big-endian u32"] # [doc = " array size."] # [doc = ""] # [doc = " Array format family stores a sequence of elements in 1, 3, or 5 bytes of extra bytes in addition"] # [doc = " to the elements."] # [doc = ""] # [doc = " # Note"] # [doc = ""] # [doc = " This function will silently retry on every EINTR received from the underlying `Read` until"] # [doc = " successful read."] pub fn read_array_len < R > (rd : & mut R) -> Result < u32 , ValueReadError < R :: Error > > where R : RmpRead , { match read_marker (rd) ? { Marker :: FixArray (size) => Ok (u32 :: from (size)) , Marker :: Array16 => Ok (u32 :: from (rd . read_data_u16 () ?)) , Marker :: Array32 => Ok (rd . read_data_u32 () ?) , marker => Err (ValueReadError :: TypeMismatch (marker)) , } }
};
}
