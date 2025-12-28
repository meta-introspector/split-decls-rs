macro_rules! deps {
    () => {
        UniverseInfo!();
        ToUniverseInfo!();
        AscribeUserTypeQuery!();
    };
}

macro_rules! impl_97 {
    () => {
        deps!();
        impl < 'tcx > ToUniverseInfo < 'tcx > for CanonicalTypeOpAscribeUserTypeGoal < 'tcx > { fn to_universe_info (self , base_universe : ty :: UniverseIndex) -> UniverseInfo < 'tcx > { UniverseInfo :: TypeOp (Rc :: new (AscribeUserTypeQuery { canonical_query : self , base_universe })) } }
    };
}

impl_97!()