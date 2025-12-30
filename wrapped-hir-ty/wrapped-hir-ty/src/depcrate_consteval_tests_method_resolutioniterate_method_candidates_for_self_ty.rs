// Generated macro for iterate_method_candidates_for_self_ty (function)
macro_rules! Depcrate_consteval_tests_method_resolutioniterate_method_candidates_for_self_ty {
() => {
// Module: crate::consteval::tests::method_resolution
// Provides: {"iterate_method_candidates_for_self_ty"}
// Dependencies: {}
# [tracing :: instrument (skip_all , fields (name = ? name))] fn iterate_method_candidates_for_self_ty (self_ty : & Canonical < Ty > , db : & dyn HirDatabase , env : Arc < TraitEnvironment > , traits_in_scope : & FxHashSet < TraitId > , visible_from_module : VisibleFromModule , name : Option < & Name > , callback : & mut dyn MethodCandidateCallback ,) -> ControlFlow < () > { let mut table = InferenceTable :: new (db , env) ; let self_ty = table . instantiate_canonical (self_ty . clone ()) ; iterate_inherent_methods (& self_ty , & mut table , name , None , None , visible_from_module , & mut | adjustments , item , is_visible | { callback . on_inherent_method (adjustments , item , is_visible) } ,) ? ; iterate_trait_method_candidates (& self_ty , & mut table , traits_in_scope , name , None , None , & mut | adjustments , item , is_visible | { callback . on_trait_method (adjustments , item , is_visible) } ,) }
};
}
