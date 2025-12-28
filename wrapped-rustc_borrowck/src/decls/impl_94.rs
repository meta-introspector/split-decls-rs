macro_rules! deps {
    () => {
        PredicateQuery!();
        ToUniverseInfo!();
        UniverseInfo!();
    };
}

macro_rules! impl_94 {
    () => {
        deps!();
        impl < 'tcx > ToUniverseInfo < 'tcx > for CanonicalTypeOpProvePredicateGoal < 'tcx > { fn to_universe_info (self , base_universe : ty :: UniverseIndex) -> UniverseInfo < 'tcx > { UniverseInfo :: TypeOp (Rc :: new (PredicateQuery { canonical_query : self , base_universe })) } }
    };
}

impl_94!();