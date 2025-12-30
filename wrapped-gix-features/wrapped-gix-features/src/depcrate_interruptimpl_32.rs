// Generated macro for impl_32 (impl)
macro_rules! Depcrate_interruptimpl_32 {
() => {
// Module: crate::interrupt
// Provides: {"impl_32"}
// Dependencies: {}
impl < W > io :: Seek for Write < '_ , W > where W : std :: io :: Seek , { fn seek (& mut self , pos : io :: SeekFrom) -> io :: Result < u64 > { self . inner . seek (pos) } }
};
}
