// Generated macro for impl_31 (impl)
macro_rules! Depcrate_alloc_alloc_bytesimpl_31 {
() => {
// Module: crate::alloc::alloc_bytes
// Provides: {"impl_31"}
// Dependencies: {}
impl Drop for MiriAllocBytes { fn drop (& mut self) { let alloc_layout = if self . layout . size () == 0 { Layout :: from_size_align (1 , self . layout . align ()) . unwrap () } else { self . layout } ; unsafe { match self . params . clone () { MiriAllocParams :: Global => alloc :: dealloc (self . ptr , alloc_layout) , MiriAllocParams :: Isolated (alloc) => alloc . borrow_mut () . dealloc (self . ptr , alloc_layout) , } } } }
};
}
