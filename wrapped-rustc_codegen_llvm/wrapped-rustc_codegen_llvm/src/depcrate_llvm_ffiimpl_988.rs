// Generated macro for impl_988 (impl)
macro_rules! Depcrate_llvm_ffiimpl_988 {
() => {
// Module: crate::llvm::ffi
// Provides: {"impl_988"}
// Dependencies: {}
impl AtomicOrdering { pub (crate) fn from_generic (ao : rustc_middle :: ty :: AtomicOrdering) -> Self { use rustc_middle :: ty :: AtomicOrdering as Common ; match ao { Common :: Relaxed => Self :: Monotonic , Common :: Acquire => Self :: Acquire , Common :: Release => Self :: Release , Common :: AcqRel => Self :: AcquireRelease , Common :: SeqCst => Self :: SequentiallyConsistent , } } }
};
}
