// Generated macro for impl_32 (impl)
macro_rules! Depcrate_graphimpl_32 {
() => {
// Module: crate::graph
// Provides: {"impl_32"}
// Dependencies: {}
impl < T > std :: fmt :: Debug for Commit < T > where T : std :: fmt :: Debug , { fn fmt (& self , f : & mut Formatter < '_ >) -> std :: fmt :: Result { f . debug_struct ("Commit") . field ("parents" , & self . parents) . field ("commit_time" , & self . commit_time) . field ("generation" , & self . generation) . field ("data" , & self . data) . finish () } }
};
}
