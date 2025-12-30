// Generated macro for implements_trait (function)
macro_rules! Depcrate_consteval_tests_method_resolutionimplements_trait {
() => {
// Module: crate::consteval::tests::method_resolution
// Provides: {"implements_trait"}
// Dependencies: {}
pub fn implements_trait (ty : & Canonical < Ty > , db : & dyn HirDatabase , env : & TraitEnvironment , trait_ : TraitId ,) -> bool { let goal = generic_implements_goal (db , env , trait_ , ty) ; let solution = db . trait_solve (env . krate , env . block , goal . cast (Interner)) ; solution . is_some () }
};
}
