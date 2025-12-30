// Generated macro for AttrStmt (enum)
macro_rules! Depcrate_astAttrStmt {
() => {
// Module: crate::ast
// Provides: {"AttrStmt"}
// Dependencies: {}
# [doc = " An attribute statement. This corresponds to the rule `attr_stmt`"] # [doc = " non-terminal of the grammar."] # [derive (Clone , Debug , Eq , PartialEq , Ord , PartialOrd , Hash)] pub enum AttrStmt < A > { # [doc = " An `AttrStmt` on the whole graph."] Graph (AttrList < A >) , # [doc = " An `AttrStmt` on each nodes of the graph."] Node (AttrList < A >) , # [doc = " An `AttrStmt` on each edges of the graph."] Edge (AttrList < A >) , }
};
}
