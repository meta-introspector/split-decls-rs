// Generated macro for impl_297 (impl)
macro_rules! Depcrate_hybrid_errorimpl_297 {
() => {
// Module: crate::hybrid::error
// Provides: {"impl_297"}
// Dependencies: {}
impl StartError { pub (crate) fn cache (err : CacheError) -> StartError { StartError :: Cache { err } } pub (crate) fn quit (byte : u8) -> StartError { StartError :: Quit { byte } } pub (crate) fn unsupported_anchored (mode : Anchored) -> StartError { StartError :: UnsupportedAnchored { mode } } }
};
}
