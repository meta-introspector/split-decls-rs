macro_rules! AscribeUserTypeQuery {
    () => {
        struct AscribeUserTypeQuery < 'tcx > { canonical_query : CanonicalTypeOpAscribeUserTypeGoal < 'tcx > , base_universe : ty :: UniverseIndex , }
    };
}

AscribeUserTypeQuery!();