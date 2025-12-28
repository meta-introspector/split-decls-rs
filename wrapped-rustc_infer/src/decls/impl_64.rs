macro_rules! deps {
    () => {
        SubregionOrigin!();
        RegionResolutionError!();
    };
}

macro_rules! impl_64 {
    () => {
        deps!();
        impl < 'tcx > RegionResolutionError < 'tcx > { pub fn origin (& self) -> & SubregionOrigin < 'tcx > { match self { RegionResolutionError :: ConcreteFailure (origin , _ , _) | RegionResolutionError :: GenericBoundFailure (origin , _ , _) | RegionResolutionError :: SubSupConflict (_ , _ , origin , _ , _ , _ , _) | RegionResolutionError :: UpperBoundUniverseConflict (_ , _ , _ , origin , _) | RegionResolutionError :: CannotNormalize (_ , origin) => origin , } } }
    };
}

impl_64!();