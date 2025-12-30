// Generated macro for impl_40 (impl)
macro_rules! Depcrate_fileimpl_40 {
() => {
// Module: crate::file
// Provides: {"impl_40"}
// Dependencies: {}
impl Read for File { fn read (& mut self , buf : & mut [u8]) -> std :: io :: Result < usize > { self . file . read (buf) . map_err (| source | self . error (source , ErrorKind :: Read)) } fn read_vectored (& mut self , bufs : & mut [std :: io :: IoSliceMut < '_ >]) -> std :: io :: Result < usize > { self . file . read_vectored (bufs) . map_err (| source | self . error (source , ErrorKind :: Read)) } }
};
}
