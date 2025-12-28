macro_rules! deps {
    () => {
        DeeplyNormalizeQuery!();
        ToUniverseInfo!();
        UniverseInfo!();
    };
}

macro_rules! impl_96 {
    () => {
        deps!();
        impl < 'tcx , T : Copy + fmt :: Display + TypeFoldable < TyCtxt < 'tcx > > + 'tcx > ToUniverseInfo < 'tcx > for CanonicalTypeOpDeeplyNormalizeGoal < 'tcx , T > { fn to_universe_info (self , base_universe : ty :: UniverseIndex) -> UniverseInfo < 'tcx > { UniverseInfo :: TypeOp (Rc :: new (DeeplyNormalizeQuery { canonical_query : self , base_universe })) } }
    };
}

impl_96!();