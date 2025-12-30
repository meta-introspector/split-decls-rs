// Generated macro for impl_681 (impl)
macro_rules! Depcrate_concurrency_genmc_helperimpl_681 {
() => {
// Module: crate::concurrency::genmc::helper
// Provides: {"impl_681"}
// Dependencies: {}
impl AtomicWriteOrd { pub (super) fn to_genmc (self) -> MemOrdering { match self { AtomicWriteOrd :: Relaxed => MemOrdering :: Relaxed , AtomicWriteOrd :: Release => MemOrdering :: Release , AtomicWriteOrd :: SeqCst => MemOrdering :: SequentiallyConsistent , } } }
};
}
