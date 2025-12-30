// Generated macro for read_str_len (function)
macro_rules! Depcrate_decode_strread_str_len {
() => {
// Module: crate::decode::str
// Provides: {"read_str_len"}
// Dependencies: {}
# [doc = " Attempts to read up to 9 bytes from the given reader and to decode them as a string `u32` size"] # [doc = " value."] # [doc = ""] # [doc = " According to the MessagePack specification, the string format family stores an byte array in 1,"] # [doc = " 2, 3, or 5 bytes of extra bytes in addition to the size of the byte array."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " This function will return `ValueReadError` on any I/O error while reading either the marker or"] # [doc = " the data."] # [doc = ""] # [doc = " It also returns `ValueReadError::TypeMismatch` if the actual type is not equal with the"] # [doc = " expected one, indicating you with the actual type."] # [inline] pub fn read_str_len < R : RmpRead > (rd : & mut R) -> Result < u32 , ValueReadError < R :: Error > > { Ok (read_str_len_with_nread (rd) ? . 0) }
};
}
