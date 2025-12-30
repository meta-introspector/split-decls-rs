// Generated macro for normalize_projection_query (function)
macro_rules! Depcrate_consteval_tests_traitsnormalize_projection_query {
() => {
// Module: crate::consteval::tests::traits
// Provides: {"normalize_projection_query"}
// Dependencies: {}
pub (crate) fn normalize_projection_query (db : & dyn HirDatabase , projection : ProjectionTy , env : Arc < TraitEnvironment > ,) -> Ty { if projection . substitution . iter (Interner) . any (| arg | { arg . ty (Interner) . is_some_and (| ty | ty . data (Interner) . flags . intersects (TypeFlags :: HAS_TY_INFER)) }) { never ! ("Invoking `normalize_projection_query` with a projection type containing inference var") ; return TyKind :: Error . intern (Interner) ; } let mut table = InferenceTable :: new (db , env) ; let ty = table . normalize_projection_ty (projection) ; table . resolve_completely (ty) }
};
}
