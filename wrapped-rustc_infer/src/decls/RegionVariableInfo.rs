macro_rules! deps {
    () => {
        RegionVariableOrigin!();
    };
}

macro_rules! RegionVariableInfo {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy)] pub struct RegionVariableInfo { pub origin : RegionVariableOrigin , pub universe : ty :: UniverseIndex , }
    };
}

RegionVariableInfo!()