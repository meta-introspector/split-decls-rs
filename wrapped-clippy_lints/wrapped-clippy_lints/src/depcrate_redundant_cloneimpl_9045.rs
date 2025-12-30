// Generated macro for impl_9045 (impl)
macro_rules! Depcrate_redundant_cloneimpl_9045 {
() => {
// Module: crate::redundant_clone
// Provides: {"impl_9045"}
// Dependencies: {}
impl From < Option < mir :: Location > > for MirLocalUsage { fn from (loc : Option < mir :: Location >) -> Self { loc . map_or (MirLocalUsage :: Unused , MirLocalUsage :: Used) } }
};
}
