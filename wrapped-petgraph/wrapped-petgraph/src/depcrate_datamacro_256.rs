// Generated macro for macro_256 (macro)
macro_rules! Depcrate_datamacro_256 {
() => {
// Module: crate::data
// Provides: {"macro_256"}
// Dependencies: {}
trait_template ! { # [doc = " Access node and edge weights (associated data)."] # [allow (clippy :: needless_arbitrary_self_type)] pub trait DataMap : Data { @ section self fn node_weight (self : & Self , id : Self :: NodeId) -> Option <& Self :: NodeWeight >; fn edge_weight (self : & Self , id : Self :: EdgeId) -> Option <& Self :: EdgeWeight >; } }
};
}
