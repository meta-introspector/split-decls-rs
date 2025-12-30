// Generated macro for impl_246 (impl)
macro_rules! Depcrate_pikevmimpl_246 {
() => {
// Module: crate::pikevm
// Provides: {"impl_246"}
// Dependencies: {}
impl Threads { fn new () -> Self { Threads { set : SparseSet :: new (0) , caps : vec ! [] , slots_per_thread : 0 , } } fn resize (& mut self , num_insts : usize , ncaps : usize) { if num_insts == self . set . capacity () { return ; } self . slots_per_thread = ncaps * 2 ; self . set = SparseSet :: new (num_insts) ; self . caps = vec ! [None ; self . slots_per_thread * num_insts] ; } fn caps (& mut self , pc : usize) -> & mut [Option < usize >] { let i = pc * self . slots_per_thread ; & mut self . caps [i .. i + self . slots_per_thread] } }
};
}
