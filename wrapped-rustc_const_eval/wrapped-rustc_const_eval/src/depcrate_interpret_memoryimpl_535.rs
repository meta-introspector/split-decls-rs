// Generated macro for impl_535 (impl)
macro_rules! Depcrate_interpret_memoryimpl_535 {
() => {
// Module: crate::interpret::memory
// Provides: {"impl_535"}
// Dependencies: {}
impl < T : MayLeak > MayLeak for MemoryKind < T > { # [inline] fn may_leak (self) -> bool { match self { MemoryKind :: Stack => false , MemoryKind :: CallerLocation => true , MemoryKind :: Machine (k) => k . may_leak () , } } }
};
}
