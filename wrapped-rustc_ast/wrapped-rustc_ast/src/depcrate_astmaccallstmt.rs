// Generated macro for MacCallStmt (struct)
macro_rules! Depcrate_astMacCallStmt {
() => {
// Module: crate::ast
// Provides: {"MacCallStmt"}
// Dependencies: {}
# [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub struct MacCallStmt { pub mac : Box < MacCall > , pub style : MacStmtStyle , pub attrs : AttrVec , pub tokens : Option < LazyAttrTokenStream > , }
};
}
