// Generated macro for impl_9277 (impl)
macro_rules! Depcrate_redundant_cloneimpl_9277 {
() => {
// Module: crate::redundant_clone
// Provides: {"impl_9277"}
// Dependencies: {}
impl From < Option < mir :: Location > > for MirLocalUsage { fn from (loc : Option < mir :: Location >) -> Self { loc . map_or (MirLocalUsage :: Unused , MirLocalUsage :: Used) } }
};
}
