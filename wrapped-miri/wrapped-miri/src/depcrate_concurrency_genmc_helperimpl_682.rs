// Generated macro for impl_682 (impl)
macro_rules! Depcrate_concurrency_genmc_helperimpl_682 {
() => {
// Module: crate::concurrency::genmc::helper
// Provides: {"impl_682"}
// Dependencies: {}
impl AtomicFenceOrd { pub (super) fn to_genmc (self) -> MemOrdering { match self { AtomicFenceOrd :: Acquire => MemOrdering :: Acquire , AtomicFenceOrd :: Release => MemOrdering :: Release , AtomicFenceOrd :: AcqRel => MemOrdering :: AcquireRelease , AtomicFenceOrd :: SeqCst => MemOrdering :: SequentiallyConsistent , } } }
};
}
