// Generated macro for impl_1300 (impl)
macro_rules! Depcrate_mm_physicalmemimpl_1300 {
() => {
// Module: crate::mm::physicalmem
// Provides: {"impl_1300"}
// Dependencies: {}
impl PageRangeExt for PageRange { fn containing (start : usize , end : usize) -> Result < Self , PageRangeError > { let start = start . align_down (free_list :: PAGE_SIZE) ; let end = end . align_up (free_list :: PAGE_SIZE) ; Self :: new (start , end) } fn and (self , rhs : Self) -> Option < Self > { let start = self . start () . max (rhs . start ()) ; let end = self . end () . min (rhs . end ()) ; Self :: new (start , end) . ok () } }
};
}
