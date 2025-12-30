// Generated macro for StaticItem (struct)
macro_rules! Depcrate_astStaticItem {
() => {
// Module: crate::ast
// Provides: {"StaticItem"}
// Dependencies: {}
# [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub struct StaticItem { pub ident : Ident , pub ty : Box < Ty > , pub safety : Safety , pub mutability : Mutability , pub expr : Option < Box < Expr > > , pub define_opaque : Option < ThinVec < (NodeId , Path) > > , }
};
}
