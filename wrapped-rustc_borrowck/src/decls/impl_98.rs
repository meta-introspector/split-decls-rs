macro_rules! deps {
    () => {
        UniverseInfo!();
        ToUniverseInfo!();
    };
}

macro_rules! impl_98 {
    () => {
        deps!();
        impl < 'tcx > ToUniverseInfo < 'tcx > for ! { fn to_universe_info (self , _base_universe : ty :: UniverseIndex) -> UniverseInfo < 'tcx > { self } }
    };
}

impl_98!()