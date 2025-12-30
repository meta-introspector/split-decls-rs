// Generated macro for Expr (struct)
macro_rules! Depcrate_astExpr {
() => {
// Module: crate::ast
// Provides: {"Expr"}
// Dependencies: {}
# [doc = " An expression."] # [derive (Clone , Encodable , Decodable , Debug)] pub struct Expr { pub id : NodeId , pub kind : ExprKind , pub span : Span , pub attrs : AttrVec , pub tokens : Option < LazyAttrTokenStream > , }
};
}
