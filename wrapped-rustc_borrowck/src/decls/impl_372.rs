macro_rules! deps {
    () => {
        RegionDefinition!();
        Representative!();
    };
}

macro_rules! impl_372 {
    () => {
        deps!();
        impl Representative { pub (crate) fn rvid (self) -> RegionVid { match self { Representative :: FreeRegion (region_vid) | Representative :: Placeholder (region_vid) | Representative :: Existential (region_vid) => region_vid , } } pub (crate) fn new (r : RegionVid , definition : & RegionDefinition < '_ >) -> Self { match definition . origin { NllRegionVariableOrigin :: FreeRegion => Representative :: FreeRegion (r) , NllRegionVariableOrigin :: Placeholder (_) => Representative :: Placeholder (r) , NllRegionVariableOrigin :: Existential { .. } => Representative :: Existential (r) , } } }
    };
}

impl_372!();