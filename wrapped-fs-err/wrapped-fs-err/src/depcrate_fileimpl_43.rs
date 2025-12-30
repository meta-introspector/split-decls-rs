// Generated macro for impl_43 (impl)
macro_rules! Depcrate_fileimpl_43 {
() => {
// Module: crate::file
// Provides: {"impl_43"}
// Dependencies: {}
impl Seek for File { fn seek (& mut self , pos : std :: io :: SeekFrom) -> std :: io :: Result < u64 > { self . file . seek (pos) . map_err (| source | self . error (source , ErrorKind :: Seek)) } }
};
}
