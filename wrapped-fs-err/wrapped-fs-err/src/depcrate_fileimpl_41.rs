// Generated macro for impl_41 (impl)
macro_rules! Depcrate_fileimpl_41 {
() => {
// Module: crate::file
// Provides: {"impl_41"}
// Dependencies: {}
impl Read for & File { fn read (& mut self , buf : & mut [u8]) -> std :: io :: Result < usize > { (& self . file) . read (buf) . map_err (| source | self . error (source , ErrorKind :: Read)) } fn read_vectored (& mut self , bufs : & mut [std :: io :: IoSliceMut < '_ >]) -> std :: io :: Result < usize > { (& self . file) . read_vectored (bufs) . map_err (| source | self . error (source , ErrorKind :: Read)) } }
};
}
