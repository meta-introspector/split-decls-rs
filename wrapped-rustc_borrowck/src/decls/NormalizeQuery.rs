macro_rules! NormalizeQuery {
    () => {
        struct NormalizeQuery < 'tcx , T > { canonical_query : CanonicalTypeOpNormalizeGoal < 'tcx , T > , base_universe : ty :: UniverseIndex , }
    };
}

NormalizeQuery!();