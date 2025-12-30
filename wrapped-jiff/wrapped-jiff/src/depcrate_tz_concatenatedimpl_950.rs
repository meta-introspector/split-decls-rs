// Generated macro for impl_950 (impl)
macro_rules! Depcrate_tz_concatenatedimpl_950 {
() => {
// Module: crate::tz::concatenated
// Provides: {"impl_950"}
// Dependencies: {}
# [cfg (all (feature = "std" , windows))] impl Read for std :: fs :: File { fn read_exact_at (& self , mut buf : & mut [u8] , mut offset : u64 ,) -> Result < () , Error > { use std :: { io , os :: windows :: fs :: FileExt } ; while ! buf . is_empty () { match self . seek_read (buf , offset) { Ok (0) => break , Ok (n) => { buf = & mut buf [n ..] ; offset = u64 :: try_from (n) . ok () . and_then (| n | n . checked_add (offset)) . ok_or_else (| | { err ! ("offset overflow when reading from `File`") }) ? ; } Err (ref e) if e . kind () == io :: ErrorKind :: Interrupted => { } Err (e) => return Err (Error :: io (e)) , } } if ! buf . is_empty () { Err (Error :: io (io :: Error :: new (io :: ErrorKind :: UnexpectedEof , "failed to fill whole buffer" ,))) } else { Ok (()) } } }
};
}
