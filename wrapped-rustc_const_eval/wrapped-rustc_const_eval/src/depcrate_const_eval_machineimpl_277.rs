// Generated macro for impl_277 (impl)
macro_rules! Depcrate_const_eval_machineimpl_277 {
() => {
// Module: crate::const_eval::machine
// Provides: {"impl_277"}
// Dependencies: {}
impl fmt :: Display for MemoryKind { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { MemoryKind :: Heap { was_made_global } => { write ! (f , "heap allocation{}" , if * was_made_global { " (made global)" } else { "" }) } } } }
};
}
