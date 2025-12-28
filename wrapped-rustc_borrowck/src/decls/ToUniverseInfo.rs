macro_rules! deps {
    () => {
        UniverseInfo!();
    };
}

macro_rules! ToUniverseInfo {
    () => {
        deps!();
        pub (crate) trait ToUniverseInfo < 'tcx > { fn to_universe_info (self , base_universe : ty :: UniverseIndex) -> UniverseInfo < 'tcx > ; }
    };
}

ToUniverseInfo!()