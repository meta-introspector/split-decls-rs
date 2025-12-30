// Generated macro for NodeAttachInfo (struct)
macro_rules! Depcrate_topo_placer_bkNodeAttachInfo {
() => {
// Module: crate::topo::placer::bk
// Provides: {"NodeAttachInfo"}
// Dependencies: {}
# [doc = " Maps the block alignment information."] struct NodeAttachInfo { # [doc = " For each node, marks which node in the row above it aligns to."] above : Vec < Option < NodeHandle > > , # [doc = " For each node, marks which node in the row below aligns to it."] below : Vec < Option < NodeHandle > > , }
};
}
