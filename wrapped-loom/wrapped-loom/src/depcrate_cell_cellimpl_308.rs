// Generated macro for impl_308 (impl)
macro_rules! Depcrate_cell_cellimpl_308 {
() => {
// Module: crate::cell::cell
// Provides: {"impl_308"}
// Dependencies: {}
impl < T : PartialOrd + Copy > PartialOrd for Cell < T > { fn partial_cmp (& self , other : & Self) -> Option < core :: cmp :: Ordering > { self . get () . partial_cmp (& other . get ()) } }
};
}
