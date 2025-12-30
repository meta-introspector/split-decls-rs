// Generated macro for EdgeStmt (struct)
macro_rules! Depcrate_astEdgeStmt {
() => {
// Module: crate::ast
// Provides: {"EdgeStmt"}
// Dependencies: {}
# [doc = " The description of an edge. This corresponds to the `edge_stmt` non-terminal"] # [doc = " of the grammar."] # [derive (Clone , Debug , Eq , PartialEq , Ord , PartialOrd , Hash)] pub struct EdgeStmt < A > { # [doc = " The origin of the edge."] pub from : Either < NodeID , Subgraph < A > > , # [doc = " The destination of the edge."] pub next : EdgeRHS < A > , # [doc = " The attributes of the edge."] pub attr : Option < AttrList < A > > , }
};
}
