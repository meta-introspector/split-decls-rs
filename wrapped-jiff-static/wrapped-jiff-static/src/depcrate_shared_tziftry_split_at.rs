// Generated macro for try_split_at (function)
macro_rules! Depcrate_shared_tziftry_split_at {
() => {
// Module: crate::shared::tzif
// Provides: {"try_split_at"}
// Dependencies: {}
# [doc = " Splits the given slice of bytes at the index given."] # [doc = ""] # [doc = " If the index is out of range (greater than `bytes.len()`) then an error is"] # [doc = " returned. The error message will include the `what` string given, which is"] # [doc = " meant to describe the thing being split."] fn try_split_at < 'b > (what : & 'static str , bytes : & 'b [u8] , at : usize ,) -> Result < (& 'b [u8] , & 'b [u8]) , Error > { if at > bytes . len () { Err (err ! ("expected at least {at} bytes for {what}, \
             but found only {} bytes" , bytes . len () ,)) } else { Ok (bytes . split_at (at)) } }
};
}
