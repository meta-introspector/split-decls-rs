// Generated macro for read_bin_len (function)
macro_rules! Depcrate_decoderead_bin_len {
() => {
// Module: crate::decode
// Provides: {"read_bin_len"}
// Dependencies: {}
# [doc = " Attempts to read up to 5 bytes from the given reader and to decode them as Binary array length."] # [doc = ""] # [doc = " # Note"] # [doc = ""] # [doc = " This function will silently retry on every EINTR received from the underlying `Read` until"] # [doc = " successful read."] pub fn read_bin_len < R : RmpRead > (rd : & mut R) -> Result < u32 , ValueReadError < R :: Error > > { match read_marker (rd) ? { Marker :: Bin8 => Ok (u32 :: from (rd . read_data_u8 () ?)) , Marker :: Bin16 => Ok (u32 :: from (rd . read_data_u16 () ?)) , Marker :: Bin32 => Ok (rd . read_data_u32 () ?) , marker => Err (ValueReadError :: TypeMismatch (marker)) , } }
};
}
