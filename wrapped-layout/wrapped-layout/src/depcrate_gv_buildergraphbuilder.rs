// Generated macro for GraphBuilder (struct)
macro_rules! Depcrate_gv_builderGraphBuilder {
() => {
// Module: crate::gv::builder
// Provides: {"GraphBuilder"}
// Dependencies: {}
# [doc = " This class constructs a visual graph from the parsed AST."] # [derive (Debug)] pub struct GraphBuilder { global_state : PropertyList , node_order : Vec < String > , nodes : HashMap < String , PropertyList > , edges : Vec < EdgeDesc > , # [doc = " Scopes that maintain the property list that changes as we enter and"] # [doc = " leave different regions of the graph."] global_attr : ScopedMap < String , String > , node_attr : ScopedMap < String , String > , edge_attr : ScopedMap < String , String > , }
};
}
