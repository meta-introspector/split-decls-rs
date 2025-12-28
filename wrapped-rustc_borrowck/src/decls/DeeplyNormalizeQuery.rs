macro_rules! DeeplyNormalizeQuery {
    () => {
        struct DeeplyNormalizeQuery < 'tcx , T > { canonical_query : CanonicalTypeOpDeeplyNormalizeGoal < 'tcx , T > , base_universe : ty :: UniverseIndex , }
    };
}

DeeplyNormalizeQuery!()