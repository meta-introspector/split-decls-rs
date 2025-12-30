// Generated macro for read_128_buf (function)
macro_rules! Depcrate_decoderead_128_buf {
() => {
// Module: crate::decode
// Provides: {"read_128_buf"}
// Dependencies: {}
fn read_128_buf < 'de , R : ReadSlice < 'de > > (rd : & mut R , len : u8) -> Result < i128 , Error > { if len != 16 { return Err (Error :: LengthMismatch (16)) ; } let buf = match read_bin_data (rd , 16) ? { Reference :: Borrowed (buf) => buf , Reference :: Copied (buf) => buf , } ; Ok (i128 :: from_be_bytes (buf . try_into () . map_err (| _ | Error :: LengthMismatch (16)) ?)) }
};
}
