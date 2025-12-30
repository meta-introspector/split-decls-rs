// Generated macro for impl_79 (impl)
macro_rules! Depcrate_blockimpl_79 {
() => {
// Module: crate::block
// Provides: {"impl_79"}
// Dependencies: {}
impl Ord for Block { # [inline] fn cmp (& self , other : & Self) -> Ordering { self . into_usize_array () . cmp (& other . into_usize_array ()) } }
};
}
