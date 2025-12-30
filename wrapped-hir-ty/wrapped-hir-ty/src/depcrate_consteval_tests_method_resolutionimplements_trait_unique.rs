// Generated macro for implements_trait_unique (function)
macro_rules! Depcrate_consteval_tests_method_resolutionimplements_trait_unique {
() => {
// Module: crate::consteval::tests::method_resolution
// Provides: {"implements_trait_unique"}
// Dependencies: {}
pub fn implements_trait_unique (ty : & Canonical < Ty > , db : & dyn HirDatabase , env : & TraitEnvironment , trait_ : TraitId ,) -> bool { let goal = generic_implements_goal (db , env , trait_ , ty) ; let solution = db . trait_solve (env . krate , env . block , goal . cast (Interner)) ; matches ! (solution , Some (crate :: Solution :: Unique (_))) }
};
}
