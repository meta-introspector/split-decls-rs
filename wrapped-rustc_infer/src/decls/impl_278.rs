macro_rules! deps {
    () => {
        RegionVariableOrigin!();
    };
}

macro_rules! impl_278 {
    () => {
        deps!();
        impl RegionVariableOrigin { pub fn span (& self) -> Span { match * self { RegionVariableOrigin :: Misc (a) | RegionVariableOrigin :: PatternRegion (a) | RegionVariableOrigin :: BorrowRegion (a) | RegionVariableOrigin :: Autoref (a) | RegionVariableOrigin :: Coercion (a) | RegionVariableOrigin :: RegionParameterDefinition (a , ..) | RegionVariableOrigin :: BoundRegion (a , ..) | RegionVariableOrigin :: UpvarRegion (_ , a) => a , RegionVariableOrigin :: Nll (..) => bug ! ("NLL variable used with `span`") , } } }
    };
}

impl_278!()