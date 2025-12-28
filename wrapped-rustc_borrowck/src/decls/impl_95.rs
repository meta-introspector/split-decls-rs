macro_rules! deps {
    () => {
        NormalizeQuery!();
        UniverseInfo!();
        ToUniverseInfo!();
    };
}

macro_rules! impl_95 {
    () => {
        deps!();
        impl < 'tcx , T : Copy + fmt :: Display + TypeFoldable < TyCtxt < 'tcx > > + 'tcx > ToUniverseInfo < 'tcx > for CanonicalTypeOpNormalizeGoal < 'tcx , T > { fn to_universe_info (self , base_universe : ty :: UniverseIndex) -> UniverseInfo < 'tcx > { UniverseInfo :: TypeOp (Rc :: new (NormalizeQuery { canonical_query : self , base_universe })) } }
    };
}

impl_95!()