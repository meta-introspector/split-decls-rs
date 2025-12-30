// Generated macro for impl_33 (impl)
macro_rules! Depcrate_graphimpl_33 {
() => {
// Module: crate::graph
// Provides: {"impl_33"}
// Dependencies: {}
impl < T > Clone for Commit < T > where T : Clone , { fn clone (& self) -> Self { Commit { parents : self . parents . clone () , commit_time : self . commit_time , generation : self . generation , data : self . data . clone () , } } }
};
}
