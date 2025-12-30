// Generated macro for impl_150 (impl)
macro_rules! Depcrate_ffi_rustimpl_150 {
() => {
// Module: crate::ffi::rust
// Provides: {"impl_150"}
// Dependencies: {}
impl From < FlushCompress > for MZFlush { fn from (value : FlushCompress) -> Self { match value { FlushCompress :: None => Self :: None , FlushCompress :: Partial | FlushCompress :: Sync => Self :: Sync , FlushCompress :: Full => Self :: Full , FlushCompress :: Finish => Self :: Finish , } } }
};
}
