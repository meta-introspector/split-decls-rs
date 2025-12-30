// Generated macro for impl_59 (impl)
macro_rules! Depcrate_tdefimpl_59 {
() => {
// Module: crate::tdef
// Provides: {"impl_59"}
// Dependencies: {}
impl From < tdefl_flush > for TDEFLFlush { fn from (flush : tdefl_flush) -> TDEFLFlush { use tdefl_flush :: * ; match flush { TDEFL_NO_FLUSH => TDEFLFlush :: None , TDEFL_SYNC_FLUSH => TDEFLFlush :: Sync , TDEFL_FULL_FLUSH => TDEFLFlush :: Full , TDEFL_FINISH => TDEFLFlush :: Finish , } } }
};
}
