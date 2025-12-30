// Generated macro for impl_35 (impl)
macro_rules! Depcrate_merge_base_functionimpl_35 {
() => {
// Module: crate::merge_base::function
// Provides: {"impl_35"}
// Dependencies: {}
impl Ord for GenThenTime { fn cmp (& self , other : & Self) -> Ordering { self . generation . cmp (& other . generation) . then (self . time . cmp (& other . time)) } }
};
}
