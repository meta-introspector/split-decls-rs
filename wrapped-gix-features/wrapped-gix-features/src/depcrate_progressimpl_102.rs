// Generated macro for impl_102 (impl)
macro_rules! Depcrate_progressimpl_102 {
() => {
// Module: crate::progress
// Provides: {"impl_102"}
// Dependencies: {}
impl < T , P > io :: Seek for Write < T , P > where T : io :: Seek , { fn seek (& mut self , pos : io :: SeekFrom) -> io :: Result < u64 > { self . inner . seek (pos) } }
};
}
