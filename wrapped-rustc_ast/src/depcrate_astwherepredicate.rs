// Generated macro for WherePredicate (struct)
macro_rules! Depcrate_astWherePredicate {
() => {
// Module: crate::ast
// Provides: {"WherePredicate"}
// Dependencies: {}
# [doc = " A single predicate in a where-clause."] # [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub struct WherePredicate { pub attrs : AttrVec , pub kind : WherePredicateKind , pub id : NodeId , pub span : Span , pub is_placeholder : bool , }
};
}
