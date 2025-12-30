// Generated macro for GenericParam (struct)
macro_rules! Depcrate_astGenericParam {
() => {
// Module: crate::ast
// Provides: {"GenericParam"}
// Dependencies: {}
# [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub struct GenericParam { pub id : NodeId , pub ident : Ident , pub attrs : AttrVec , # [visitable (extra = BoundKind :: Bound)] pub bounds : GenericBounds , pub is_placeholder : bool , pub kind : GenericParamKind , pub colon_span : Option < Span > , }
};
}
