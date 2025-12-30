// Generated macro for macro_451 (macro)
macro_rules! Depcrate_node_idmacro_451 {
() => {
// Module: crate::node_id
// Provides: {"macro_451"}
// Dependencies: {}
rustc_index :: newtype_index ! { # [doc = " Identifies an AST node."] # [doc = ""] # [doc = " This identifies top-level definitions, expressions, and everything in between."] # [doc = " This is later turned into [`DefId`] and `HirId` for the HIR."] # [doc = ""] # [doc = " [`DefId`]: rustc_span::def_id::DefId"] # [encodable] # [orderable] # [debug_format = "NodeId({})"] pub struct NodeId { # [doc = " The [`NodeId`] used to represent the root of the crate."] const CRATE_NODE_ID = 0 ; } }
};
}
