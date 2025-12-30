// Generated macro for iterate_method_candidates_dyn (function)
macro_rules! Depcrate_consteval_tests_method_resolutioniterate_method_candidates_dyn {
() => {
// Module: crate::consteval::tests::method_resolution
// Provides: {"iterate_method_candidates_dyn"}
// Dependencies: {}
pub fn iterate_method_candidates_dyn (ty : & Canonical < Ty > , db : & dyn HirDatabase , env : Arc < TraitEnvironment > , traits_in_scope : & FxHashSet < TraitId > , visible_from_module : VisibleFromModule , name : Option < & Name > , mode : LookupMode , callback : & mut dyn MethodCandidateCallback ,) -> ControlFlow < () > { let _p = tracing :: info_span ! ("iterate_method_candidates_dyn" , ? mode , ? name , traits_in_scope_len = traits_in_scope . len ()) . entered () ; match mode { LookupMode :: MethodCall => { let mut table = InferenceTable :: new (db , env) ; let ty = table . instantiate_canonical (ty . clone ()) ; let deref_chain = autoderef_method_receiver (& mut table , ty) ; deref_chain . into_iter () . try_for_each (| (receiver_ty , adj) | { iterate_method_candidates_with_autoref (& mut table , receiver_ty , adj , traits_in_scope , visible_from_module , name , callback ,) }) } LookupMode :: Path => { iterate_method_candidates_for_self_ty (ty , db , env , traits_in_scope , visible_from_module , name , callback ,) } } }
};
}
