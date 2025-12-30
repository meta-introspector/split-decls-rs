// Generated macro for lookup_impl_method_query (function)
macro_rules! Depcrate_consteval_tests_method_resolutionlookup_impl_method_query {
() => {
// Module: crate::consteval::tests::method_resolution
// Provides: {"lookup_impl_method_query"}
// Dependencies: {}
# [doc = " Looks up the impl method that actually runs for the trait method `func`."] # [doc = ""] # [doc = " Returns `func` if it's not a method defined in a trait or the lookup failed."] pub (crate) fn lookup_impl_method_query < 'db > (db : & 'db dyn HirDatabase , env : Arc < TraitEnvironment < 'db > > , func : FunctionId , fn_subst : GenericArgs < 'db > ,) -> (FunctionId , GenericArgs < 'db >) { let interner = DbInterner :: new_with (db , Some (env . krate) , env . block) ; let infcx = interner . infer_ctxt () . build (TypingMode :: PostAnalysis) ; let ItemContainerId :: TraitId (trait_id) = func . loc (db) . container else { return (func , fn_subst) ; } ; let trait_params = db . generic_params (trait_id . into ()) . len () ; let trait_ref = TraitRef :: new (interner , trait_id . into () , GenericArgs :: new_from_iter (interner , fn_subst . iter () . take (trait_params)) ,) ; let name = & db . function_signature (func) . name ; let Some ((impl_fn , impl_subst)) = lookup_impl_assoc_item_for_trait_ref (& infcx , trait_ref , env , name) . and_then (| assoc | { if let (AssocItemId :: FunctionId (id) , subst) = assoc { Some ((id , subst)) } else { None } }) else { return (func , fn_subst) ; } ; (impl_fn , GenericArgs :: new_from_iter (interner , impl_subst . iter () . chain (fn_subst . iter () . skip (trait_params)) ,) ,) }
};
}
