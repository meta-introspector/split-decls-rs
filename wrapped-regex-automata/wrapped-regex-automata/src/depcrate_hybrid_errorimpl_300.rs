// Generated macro for impl_300 (impl)
macro_rules! Depcrate_hybrid_errorimpl_300 {
() => {
// Module: crate::hybrid::error
// Provides: {"impl_300"}
// Dependencies: {}
impl StartError { pub (crate) fn cache (err : CacheError) -> StartError { StartError :: Cache { err } } pub (crate) fn quit (byte : u8) -> StartError { StartError :: Quit { byte } } pub (crate) fn unsupported_anchored (mode : Anchored) -> StartError { StartError :: UnsupportedAnchored { mode } } }
};
}
