// Generated macro for iterate_path_candidates (function)
macro_rules! Depcrate_consteval_tests_method_resolutioniterate_path_candidates {
() => {
// Module: crate::consteval::tests::method_resolution
// Provides: {"iterate_path_candidates"}
// Dependencies: {}
pub fn iterate_path_candidates (ty : & Canonical < Ty > , db : & dyn HirDatabase , env : Arc < TraitEnvironment > , traits_in_scope : & FxHashSet < TraitId > , visible_from_module : VisibleFromModule , name : Option < & Name > , callback : & mut dyn MethodCandidateCallback ,) -> ControlFlow < () > { iterate_method_candidates_dyn (ty , db , env , traits_in_scope , visible_from_module , name , LookupMode :: Path , callback ,) }
};
}
