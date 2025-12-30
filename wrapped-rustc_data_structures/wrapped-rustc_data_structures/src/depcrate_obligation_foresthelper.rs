// Generated macro for helper (module)
macro_rules! Depcrate_obligation_foresthelper {
() => {
// Module: crate::obligation_forest
// Provides: {"helper"}
// Dependencies: {}
mod helper { use super :: * ; pub (super) type ObligationTreeIdGenerator = impl Iterator < Item = ObligationTreeId > ; impl < O : ForestObligation > ObligationForest < O > { # [define_opaque (ObligationTreeIdGenerator)] pub fn new () -> ObligationForest < O > { ObligationForest { nodes : vec ! [] , done_cache : Default :: default () , active_cache : Default :: default () , reused_node_vec : vec ! [] , obligation_tree_id_generator : (0 ..) . map (ObligationTreeId) , error_cache : Default :: default () , } } } }
};
}
