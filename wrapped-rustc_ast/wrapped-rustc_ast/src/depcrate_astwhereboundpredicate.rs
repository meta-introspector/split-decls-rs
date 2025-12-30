// Generated macro for WhereBoundPredicate (struct)
macro_rules! Depcrate_astWhereBoundPredicate {
() => {
// Module: crate::ast
// Provides: {"WhereBoundPredicate"}
// Dependencies: {}
# [doc = " A type bound."] # [doc = ""] # [doc = " E.g., `for<'c> Foo: Send + Clone + 'c`."] # [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub struct WhereBoundPredicate { # [doc = " Any generics from a `for` binding."] pub bound_generic_params : ThinVec < GenericParam > , # [doc = " The type being bounded."] pub bounded_ty : Box < Ty > , # [doc = " Trait and lifetime bounds (`Clone + Send + 'static`)."] # [visitable (extra = BoundKind :: Bound)] pub bounds : GenericBounds , }
};
}
