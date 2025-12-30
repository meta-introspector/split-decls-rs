// Generated macro for WhereRegionPredicate (struct)
macro_rules! Depcrate_astWhereRegionPredicate {
() => {
// Module: crate::ast
// Provides: {"WhereRegionPredicate"}
// Dependencies: {}
# [doc = " A lifetime predicate."] # [doc = ""] # [doc = " E.g., `'a: 'b + 'c`."] # [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub struct WhereRegionPredicate { # [visitable (extra = LifetimeCtxt :: Bound)] pub lifetime : Lifetime , # [visitable (extra = BoundKind :: Bound)] pub bounds : GenericBounds , }
};
}
