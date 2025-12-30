// Generated macro for NodeStmt (struct)
macro_rules! Depcrate_astNodeStmt {
() => {
// Module: crate::ast
// Provides: {"NodeStmt"}
// Dependencies: {}
# [doc = " This structure corresponds to the `node_stmt` non-terminal of the grammar."] # [doc = " It is basically a node identifier attached to some attributes."] # [derive (Clone , Debug , Eq , PartialEq , Ord , PartialOrd , Hash)] pub struct NodeStmt < A > { # [doc = " The identifier of the node."] pub node : NodeID , # [doc = " The possible list of attributes."] pub attr : Option < AttrList < A > > , }
};
}
