// Generated macro for EdgeRHS (struct)
macro_rules! Depcrate_astEdgeRHS {
() => {
// Module: crate::ast
// Provides: {"EdgeRHS"}
// Dependencies: {}
# [doc = " The Right hand side of an edge description. This corresponds to the"] # [doc = " `EdgeRHS` non-terminal of the grammar."] # [doc = " Notice that the grammar allows multiple EdgeRHS in sequence, to chain edges:"] # [doc = " `A -> B -> C`."] # [derive (Clone , Debug , Eq , PartialEq , Ord , PartialOrd , Hash)] pub struct EdgeRHS < A > { # [doc = " The identifier of the destination of the edge."] pub to : Either < NodeID , Subgraph < A > > , # [doc = " A possible chained RHS."] pub next : Option < Box < EdgeRHS < A > > > , }
};
}
