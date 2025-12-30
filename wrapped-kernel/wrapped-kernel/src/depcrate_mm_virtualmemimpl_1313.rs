// Generated macro for impl_1313 (impl)
macro_rules! Depcrate_mm_virtualmemimpl_1313 {
() => {
// Module: crate::mm::virtualmem
// Provides: {"impl_1313"}
// Dependencies: {}
impl fmt :: Display for PageAlloc { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let free_list = KERNEL_FREE_LIST . lock () ; write ! (f , "PageAlloc free list:\n{free_list}") } }
};
}
