// Generated macro for scc (function)
macro_rules! Depcrate_algo_scc_kosaraju_sccscc {
() => {
// Module: crate::algo::scc::kosaraju_scc
// Provides: {"scc"}
// Dependencies: {}
# [doc = " Renamed to `kosaraju_scc`."] # [deprecated (note = "renamed to kosaraju_scc")] pub fn scc < G > (g : G) -> Vec < Vec < G :: NodeId > > where G : IntoNeighborsDirected + Visitable + IntoNodeIdentifiers , { kosaraju_scc (g) }
};
}
