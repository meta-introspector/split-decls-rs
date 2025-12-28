macro_rules! deps {
    () => {
        SolveContext!();
        InferredIndex!();
        ConstraintContext!();
    };
}

macro_rules! solve_constraints {
    () => {
        deps!();
        pub (crate) fn solve_constraints < 'tcx > (constraints_cx : ConstraintContext < '_ , 'tcx > ,) -> ty :: CrateVariancesMap < 'tcx > { let ConstraintContext { terms_cx , constraints , .. } = constraints_cx ; let mut solutions = vec ! [ty :: Bivariant ; terms_cx . inferred_terms . len ()] ; for (id , variances) in & terms_cx . lang_items { let InferredIndex (start) = terms_cx . inferred_starts [id] ; for (i , & variance) in variances . iter () . enumerate () { solutions [start + i] = variance ; } } let mut solutions_cx = SolveContext { terms_cx , constraints , solutions } ; solutions_cx . solve () ; let variances = solutions_cx . create_map () ; ty :: CrateVariancesMap { variances } }
    };
}

solve_constraints!();