// Generated macro for Pat (struct)
macro_rules! Depcrate_astPat {
() => {
// Module: crate::ast
// Provides: {"Pat"}
// Dependencies: {}
# [doc = " A match pattern."] # [doc = ""] # [doc = " Patterns appear in match statements and some other contexts, such as `let` and `if let`."] # [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub struct Pat { pub id : NodeId , pub kind : PatKind , pub span : Span , pub tokens : Option < LazyAttrTokenStream > , }
};
}
