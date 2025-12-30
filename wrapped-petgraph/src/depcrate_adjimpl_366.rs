// Generated macro for impl_366 (impl)
macro_rules! Depcrate_adjimpl_366 {
() => {
// Module: crate::adj
// Provides: {"impl_366"}
// Dependencies: {}
impl < E , Ix : IndexType > NodeCount for List < E , Ix > { # [doc = " Returns the number of nodes in the list"] # [doc = ""] # [doc = " Computes in **O(1)** time."] fn node_count (& self) -> usize { self . suc . len () } }
};
}
