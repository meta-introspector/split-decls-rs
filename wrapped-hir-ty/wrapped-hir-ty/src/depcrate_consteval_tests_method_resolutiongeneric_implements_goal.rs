// Generated macro for generic_implements_goal (function)
macro_rules! Depcrate_consteval_tests_method_resolutiongeneric_implements_goal {
() => {
// Module: crate::consteval::tests::method_resolution
// Provides: {"generic_implements_goal"}
// Dependencies: {}
# [doc = " This creates Substs for a trait with the given Self type and type variables"] # [doc = " for all other parameters, to query Chalk with it."] # [tracing :: instrument (skip_all)] fn generic_implements_goal (db : & dyn HirDatabase , env : & TraitEnvironment , trait_ : TraitId , self_ty : & Canonical < Ty > ,) -> Canonical < InEnvironment < super :: DomainGoal > > { let binders = self_ty . binders . interned () ; let trait_ref = TyBuilder :: trait_ref (db , trait_) . push (self_ty . value . clone ()) . fill_with_bound_vars (DebruijnIndex :: INNERMOST , binders . len ()) . build () ; let kinds = binders . iter () . cloned () . chain (trait_ref . substitution . iter (Interner) . skip (1) . map (| it | { let vk = match it . data (Interner) { GenericArgData :: Ty (_) => VariableKind :: Ty (chalk_ir :: TyVariableKind :: General) , GenericArgData :: Lifetime (_) => VariableKind :: Lifetime , GenericArgData :: Const (c) => VariableKind :: Const (c . data (Interner) . ty . clone ()) , } ; WithKind :: new (vk , UniverseIndex :: ROOT) })) ; let binders = CanonicalVarKinds :: from_iter (Interner , kinds) ; let obligation = trait_ref . cast (Interner) ; let value = InEnvironment :: new (& env . env , obligation) ; Canonical { binders , value } }
};
}
