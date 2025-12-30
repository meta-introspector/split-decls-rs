// Generated macro for VisualGraph (struct)
macro_rules! Depcrate_topo_layoutVisualGraph {
() => {
// Module: crate::topo::layout
// Provides: {"VisualGraph"}
// Dependencies: {}
# [derive (Debug)] pub struct VisualGraph { nodes : Vec < Element > , edges : Vec < (Arrow , Vec < NodeHandle >) > , self_edges : Vec < (Arrow , NodeHandle) > , pub dag : DAG , orientation : Orientation , }
};
}
