// Generated macro for impl_536 (impl)
macro_rules! Depcrate_interpret_memoryimpl_536 {
() => {
// Module: crate::interpret::memory
// Provides: {"impl_536"}
// Dependencies: {}
impl < T : fmt :: Display > fmt :: Display for MemoryKind < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { MemoryKind :: Stack => write ! (f , "stack variable") , MemoryKind :: CallerLocation => write ! (f , "caller location") , MemoryKind :: Machine (m) => write ! (f , "{m}") , } } }
};
}
