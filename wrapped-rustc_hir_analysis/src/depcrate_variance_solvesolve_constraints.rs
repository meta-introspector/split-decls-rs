// Generated macro for solve_constraints (function)
macro_rules! Depcrate_variance_solvesolve_constraints {
() => {
// Module: crate::variance::solve
// Provides: {"solve_constraints"}
// Dependencies: {}
pub (crate) fn solve_constraints < 'tcx > (constraints_cx : ConstraintContext < '_ , 'tcx > ,) -> ty :: CrateVariancesMap < 'tcx > { let ConstraintContext { terms_cx , constraints , .. } = constraints_cx ; let mut solutions = vec ! [ty :: Bivariant ; terms_cx . inferred_terms . len ()] ; for (id , variances) in & terms_cx . lang_items { let InferredIndex (start) = terms_cx . inferred_starts [id] ; for (i , & variance) in variances . iter () . enumerate () { solutions [start + i] = variance ; } } let mut solutions_cx = SolveContext { terms_cx , constraints , solutions } ; solutions_cx . solve () ; let variances = solutions_cx . create_map () ; ty :: CrateVariancesMap { variances } }
};
}
