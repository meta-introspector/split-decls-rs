// Generated macro for AsLockedWrite (trait)
macro_rules! Depcrate_streamAsLockedWrite {
() => {
// Module: crate::stream
// Provides: {"AsLockedWrite"}
// Dependencies: {}
# [doc = " Lock a stream"] pub trait AsLockedWrite : private :: Sealed { # [doc = " Locked writer type"] type Write < 'w > : RawStream + 'w where Self : 'w ; # [doc = " Lock a stream"] fn as_locked_write (& mut self) -> Self :: Write < '_ > ; }
};
}
