// Generated macro for impl_951 (impl)
macro_rules! Depcrate_tz_concatenatedimpl_951 {
() => {
// Module: crate::tz::concatenated
// Provides: {"impl_951"}
// Dependencies: {}
# [cfg (all (feature = "std" , all (not (unix) , not (windows))))] impl Read for std :: fs :: File { fn read_exact_at (& self , buf : & mut [u8] , offset : u64) -> Result < () , Error > { use std :: io :: { Read as _ , Seek as _ , SeekFrom } ; let mut file = self ; file . seek (SeekFrom :: Start (offset)) . map_err (Error :: io) . with_context (| | err ! ("failed to seek to offset {offset} in `File`") ,) ? ; file . read_exact (buf) . map_err (Error :: io) } }
};
}
