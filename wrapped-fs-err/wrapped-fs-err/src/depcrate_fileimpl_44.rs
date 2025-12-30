// Generated macro for impl_44 (impl)
macro_rules! Depcrate_fileimpl_44 {
() => {
// Module: crate::file
// Provides: {"impl_44"}
// Dependencies: {}
impl Seek for & File { fn seek (& mut self , pos : std :: io :: SeekFrom) -> std :: io :: Result < u64 > { (& self . file) . seek (pos) . map_err (| source | self . error (source , ErrorKind :: Seek)) } }
};
}
