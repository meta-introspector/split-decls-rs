// Generated macro for Node (struct)
macro_rules! Depcrate_canonicalNode {
() => {
// Module: crate::canonical
// Provides: {"Node"}
// Dependencies: {}
# [doc = " A single node of the graph."] # [derive (Debug , Clone)] pub struct Node < A > { # [doc = " The identifier of the node."] pub id : String , # [doc = " The port of the node."] pub port : Option < Port > , # [doc = " The attributes that apply to this node."] pub attr : AList < A > , }
};
}
