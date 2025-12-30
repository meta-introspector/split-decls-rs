// Generated macro for Subgraph (struct)
macro_rules! Depcrate_astSubgraph {
() => {
// Module: crate::ast
// Provides: {"Subgraph"}
// Dependencies: {}
# [doc = " A subgraph. This corresponds to the `subgraph` non-terminal of the grammar."] # [derive (Clone , Debug , Eq , PartialEq , Ord , PartialOrd , Hash)] pub struct Subgraph < A > { # [doc = " The (optional) identifier of the subgraph."] pub id : Option < String > , # [doc = " The statements that describe the subgraph."] pub stmts : StmtList < A > , }
};
}
