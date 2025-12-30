// Generated macro for impl_128 (impl)
macro_rules! Depcrate_read_read_cacheimpl_128 {
() => {
// Module: crate::read::read_cache
// Provides: {"impl_128"}
// Dependencies: {}
# [cfg (feature = "std")] impl < T : Read + Seek > ReadCacheOps for T { fn len (& mut self) -> Result < u64 , () > { self . seek (SeekFrom :: End (0)) . map_err (| _ | ()) } fn seek (& mut self , pos : u64) -> Result < u64 , () > { self . seek (SeekFrom :: Start (pos)) . map_err (| _ | ()) } fn read (& mut self , buf : & mut [u8]) -> Result < usize , () > { Read :: read (self , buf) . map_err (| _ | ()) } fn read_exact (& mut self , buf : & mut [u8]) -> Result < () , () > { Read :: read_exact (self , buf) . map_err (| _ | ()) } }
};
}
