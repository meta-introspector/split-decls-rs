// Generated macro for TyPat (struct)
macro_rules! Depcrate_astTyPat {
() => {
// Module: crate::ast
// Provides: {"TyPat"}
// Dependencies: {}
# [doc = " A pattern type pattern."] # [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub struct TyPat { pub id : NodeId , pub kind : TyPatKind , pub span : Span , pub tokens : Option < LazyAttrTokenStream > , }
};
}
