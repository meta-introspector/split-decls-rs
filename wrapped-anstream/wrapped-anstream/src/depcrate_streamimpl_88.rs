// Generated macro for impl_88 (impl)
macro_rules! Depcrate_streamimpl_88 {
() => {
// Module: crate::stream
// Provides: {"impl_88"}
// Dependencies: {}
impl AsLockedWrite for std :: io :: Stdout { type Write < 'w > = std :: io :: StdoutLock < 'w > ; # [inline] fn as_locked_write (& mut self) -> Self :: Write < '_ > { self . lock () } }
};
}
