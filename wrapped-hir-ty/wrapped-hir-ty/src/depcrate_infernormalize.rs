// Generated macro for normalize (function)
macro_rules! Depcrate_infernormalize {
() => {
// Module: crate::infer
// Provides: {"normalize"}
// Dependencies: {}
# [doc = " Fully normalize all the types found within `ty` in context of `owner` body definition."] # [doc = ""] # [doc = " This is appropriate to use only after type-check: it assumes"] # [doc = " that normalization will succeed, for example."] pub (crate) fn normalize (db : & dyn HirDatabase , trait_env : Arc < TraitEnvironment > , ty : Ty) -> Ty { if ! ty . data (Interner) . flags . intersects (TypeFlags :: HAS_PROJECTION) && ! matches ! (ty . kind (Interner) , TyKind :: Array (..)) { return ty ; } let mut table = unify :: InferenceTable :: new (db , trait_env) ; let ty_with_vars = table . normalize_associated_types_in (ty) ; table . resolve_obligations_as_possible () ; table . propagate_diverging_flag () ; table . resolve_completely (ty_with_vars) }
};
}
