// Generated macro for EdgeStmt (struct)
macro_rules! Depcrate_subgraph_freeEdgeStmt {
() => {
// Module: crate::subgraph_free
// Provides: {"EdgeStmt"}
// Dependencies: {}
# [doc = " The description of an edge. This corresponds to the `edge_stmt` non-terminal"] # [doc = " of the grammar."] # [derive (Debug , Eq , PartialEq , Ord , PartialOrd , Hash)] pub struct EdgeStmt < A > { # [doc = " The origin of the edge."] pub from : NodeID , # [doc = " The destination of the edge."] pub next : EdgeRHS , # [doc = " The attributes of the edge."] pub attr : Option < AttrList < A > > , }
};
}
