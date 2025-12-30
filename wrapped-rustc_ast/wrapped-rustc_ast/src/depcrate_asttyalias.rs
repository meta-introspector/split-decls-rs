// Generated macro for TyAlias (struct)
macro_rules! Depcrate_astTyAlias {
() => {
// Module: crate::ast
// Provides: {"TyAlias"}
// Dependencies: {}
# [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub struct TyAlias { pub defaultness : Defaultness , pub ident : Ident , pub generics : Generics , pub where_clauses : TyAliasWhereClauses , # [visitable (extra = BoundKind :: Bound)] pub bounds : GenericBounds , pub ty : Option < Box < Ty > > , }
};
}
