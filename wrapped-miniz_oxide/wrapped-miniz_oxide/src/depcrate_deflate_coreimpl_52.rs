// Generated macro for impl_52 (impl)
macro_rules! Depcrate_deflate_coreimpl_52 {
() => {
// Module: crate::deflate::core
// Provides: {"impl_52"}
// Dependencies: {}
impl From < MZFlush > for TDEFLFlush { fn from (flush : MZFlush) -> Self { match flush { MZFlush :: None => TDEFLFlush :: None , MZFlush :: Partial => TDEFLFlush :: Partial , MZFlush :: Sync => TDEFLFlush :: Sync , MZFlush :: Full => TDEFLFlush :: Full , MZFlush :: Finish => TDEFLFlush :: Finish , _ => TDEFLFlush :: None , } } }
};
}
