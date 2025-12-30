// Generated macro for implements_trait_unique (function)
macro_rules! Depcrate_consteval_tests_traitsimplements_trait_unique {
() => {
// Module: crate::consteval::tests::traits
// Provides: {"implements_trait_unique"}
// Dependencies: {}
# [doc = " This should not be used in `hir-ty`, only in `hir`."] pub fn implements_trait_unique < 'db > (ty : Ty < 'db > , db : & 'db dyn HirDatabase , env : Arc < TraitEnvironment < 'db > > , trait_ : TraitId ,) -> bool { implements_trait_unique_impl (db , env , trait_ , & mut | infcx | { infcx . fill_rest_fresh_args (trait_ . into () , [ty . into ()]) }) }
};
}
