// Generated macro for read_str_ref (function)
macro_rules! Depcrate_decode_strread_str_ref {
() => {
// Module: crate::decode::str
// Provides: {"read_str_ref"}
// Dependencies: {}
# [doc = " Attempts to read and decode a string value from the reader, returning a borrowed slice from it."] # [doc = ""] # [deprecated (since = "0.8.6" , note = "useless, use `read_str_from_slice` instead")] pub fn read_str_ref (rd : & [u8]) -> Result < & [u8] , DecodeStringError < '_ , super :: bytes :: BytesReadError > > { let mut cur = super :: Bytes :: new (rd) ; let len = read_str_len (& mut cur) ? ; Ok (& cur . remaining_slice () [.. len as usize]) }
};
}
