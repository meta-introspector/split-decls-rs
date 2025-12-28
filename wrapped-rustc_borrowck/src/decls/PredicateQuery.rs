macro_rules! PredicateQuery {
    () => {
        struct PredicateQuery < 'tcx > { canonical_query : CanonicalTypeOpProvePredicateGoal < 'tcx > , base_universe : ty :: UniverseIndex , }
    };
}

PredicateQuery!()