// Generated macro for impl_90 (impl)
macro_rules! Depcrate_streamimpl_90 {
() => {
// Module: crate::stream
// Provides: {"impl_90"}
// Dependencies: {}
impl AsLockedWrite for std :: io :: Stderr { type Write < 'w > = std :: io :: StderrLock < 'w > ; # [inline] fn as_locked_write (& mut self) -> Self :: Write < '_ > { self . lock () } }
};
}
