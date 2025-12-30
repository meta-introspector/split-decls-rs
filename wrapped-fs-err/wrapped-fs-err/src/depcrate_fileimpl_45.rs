// Generated macro for impl_45 (impl)
macro_rules! Depcrate_fileimpl_45 {
() => {
// Module: crate::file
// Provides: {"impl_45"}
// Dependencies: {}
impl Write for File { fn write (& mut self , buf : & [u8]) -> std :: io :: Result < usize > { self . file . write (buf) . map_err (| source | self . error (source , ErrorKind :: Write)) } fn write_vectored (& mut self , bufs : & [std :: io :: IoSlice < '_ >]) -> std :: io :: Result < usize > { self . file . write_vectored (bufs) . map_err (| source | self . error (source , ErrorKind :: Write)) } fn flush (& mut self) -> std :: io :: Result < () > { self . file . flush () . map_err (| source | self . error (source , ErrorKind :: Flush)) } }
};
}
