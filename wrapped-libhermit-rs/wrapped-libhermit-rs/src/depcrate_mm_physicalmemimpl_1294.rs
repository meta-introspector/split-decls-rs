// Generated macro for impl_1294 (impl)
macro_rules! Depcrate_mm_physicalmemimpl_1294 {
() => {
// Module: crate::mm::physicalmem
// Provides: {"impl_1294"}
// Dependencies: {}
impl fmt :: Display for FrameAlloc { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let free_list = PHYSICAL_FREE_LIST . lock () ; write ! (f , "FrameAlloc free list:\n{free_list}") } }
};
}
