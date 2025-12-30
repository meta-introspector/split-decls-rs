// Generated macro for impl_232 (impl)
macro_rules! Depcrate_builderimpl_232 {
() => {
// Module: crate::builder
// Provides: {"impl_232"}
// Dependencies: {}
impl ToGccOrdering for AtomicOrdering { fn to_gcc (self) -> i32 { use MemOrdering :: * ; let ordering = match self { AtomicOrdering :: Relaxed => __ATOMIC_RELAXED , AtomicOrdering :: Acquire => __ATOMIC_ACQUIRE , AtomicOrdering :: Release => __ATOMIC_RELEASE , AtomicOrdering :: AcqRel => __ATOMIC_ACQ_REL , AtomicOrdering :: SeqCst => __ATOMIC_SEQ_CST , } ; ordering as i32 } }
};
}
