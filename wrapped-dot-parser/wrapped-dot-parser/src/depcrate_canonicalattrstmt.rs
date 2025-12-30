// Generated macro for AttrStmt (enum)
macro_rules! Depcrate_canonicalAttrStmt {
() => {
// Module: crate::canonical
// Provides: {"AttrStmt"}
// Dependencies: {}
# [derive (Debug , Copy , Clone)] # [doc = " An `AttrStmt`, i.e. a statement that applies to either the whole graph, all"] # [doc = " edges, or all nodes. Note that, in a canonical graph, `AttrStmt`s contain a"] # [doc = " single statement."] pub enum AttrStmt < A > { # [doc = " An `AttrStmt` that applies to the whole graph."] Graph (A) , # [doc = " An `AttrStmt` that applies to all nodes of the graph."] Node (A) , # [doc = " An `AttrStmt` that applies to all edges of the graph."] Edge (A) , }
};
}
