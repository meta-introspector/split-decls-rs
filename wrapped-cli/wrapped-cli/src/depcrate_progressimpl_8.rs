// Generated macro for impl_8 (impl)
macro_rules! Depcrate_progressimpl_8 {
() => {
// Module: crate::progress
// Provides: {"impl_8"}
// Dependencies: {}
impl < R : Read > Read for ProgressMonitor < R > { fn read (& mut self , buf : & mut [u8]) -> std :: io :: Result < usize > { let out = self . reader . read (buf) ? ; self . read += out ; self . update (out as u64) ; Ok (out) } }
};
}
