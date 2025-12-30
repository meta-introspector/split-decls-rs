// Generated macro for ConstItem (struct)
macro_rules! Depcrate_astConstItem {
() => {
// Module: crate::ast
// Provides: {"ConstItem"}
// Dependencies: {}
# [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub struct ConstItem { pub defaultness : Defaultness , pub ident : Ident , pub generics : Generics , pub ty : Box < Ty > , pub expr : Option < Box < Expr > > , pub define_opaque : Option < ThinVec < (NodeId , Path) > > , }
};
}
