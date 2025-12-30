// Generated macro for StructExpr (struct)
macro_rules! Depcrate_astStructExpr {
() => {
// Module: crate::ast
// Provides: {"StructExpr"}
// Dependencies: {}
# [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub struct StructExpr { pub qself : Option < Box < QSelf > > , pub path : Path , pub fields : ThinVec < ExprField > , pub rest : StructRest , }
};
}
