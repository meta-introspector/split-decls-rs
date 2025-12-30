// Generated macro for impl_309 (impl)
macro_rules! Depcrate_cell_cellimpl_309 {
() => {
// Module: crate::cell::cell
// Provides: {"impl_309"}
// Dependencies: {}
impl < T : Ord + Copy > Ord for Cell < T > { fn cmp (& self , other : & Self) -> core :: cmp :: Ordering { self . get () . cmp (& other . get ()) } }
};
}
