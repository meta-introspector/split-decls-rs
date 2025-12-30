// Generated macro for impl_680 (impl)
macro_rules! Depcrate_concurrency_genmc_helperimpl_680 {
() => {
// Module: crate::concurrency::genmc::helper
// Provides: {"impl_680"}
// Dependencies: {}
impl AtomicReadOrd { pub (super) fn to_genmc (self) -> MemOrdering { match self { AtomicReadOrd :: Relaxed => MemOrdering :: Relaxed , AtomicReadOrd :: Acquire => MemOrdering :: Acquire , AtomicReadOrd :: SeqCst => MemOrdering :: SequentiallyConsistent , } } }
};
}
