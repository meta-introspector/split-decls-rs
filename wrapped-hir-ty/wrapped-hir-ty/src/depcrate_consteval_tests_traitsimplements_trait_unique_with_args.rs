// Generated macro for implements_trait_unique_with_args (function)
macro_rules! Depcrate_consteval_tests_traitsimplements_trait_unique_with_args {
() => {
// Module: crate::consteval::tests::traits
// Provides: {"implements_trait_unique_with_args"}
// Dependencies: {}
# [doc = " This should not be used in `hir-ty`, only in `hir`."] pub fn implements_trait_unique_with_args < 'db > (db : & 'db dyn HirDatabase , env : Arc < TraitEnvironment < 'db > > , trait_ : TraitId , args : GenericArgs < 'db > ,) -> bool { implements_trait_unique_impl (db , env , trait_ , & mut | _ | args) }
};
}
