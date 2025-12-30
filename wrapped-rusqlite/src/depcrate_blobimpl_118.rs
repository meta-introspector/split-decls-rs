// Generated macro for impl_118 (impl)
macro_rules! Depcrate_blobimpl_118 {
() => {
// Module: crate::blob
// Provides: {"impl_118"}
// Dependencies: {}
impl io :: Seek for Blob < '_ > { # [doc = " Seek to an offset, in bytes, in BLOB."] # [inline] fn seek (& mut self , pos : io :: SeekFrom) -> io :: Result < u64 > { let pos = match pos { io :: SeekFrom :: Start (offset) => offset as i64 , io :: SeekFrom :: Current (offset) => i64 :: from (self . pos) + offset , io :: SeekFrom :: End (offset) => i64 :: from (self . size ()) + offset , } ; if pos < 0 { Err (io :: Error :: new (io :: ErrorKind :: InvalidInput , "invalid seek to negative position" ,)) } else if pos > i64 :: from (self . size ()) { Err (io :: Error :: new (io :: ErrorKind :: InvalidInput , "invalid seek to position past end of blob" ,)) } else { self . pos = pos as i32 ; Ok (pos as u64) } } }
};
}
