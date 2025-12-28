macro_rules! deps {
    () => {
        FxIndexMap!();
        ScopeDef!();
    };
}

macro_rules! ScopeNames {
    () => {
        deps!();
        # [derive (Default)] struct ScopeNames { map : FxIndexMap < Name , SmallVec < ScopeDef , 1 > > , }
    };
}

ScopeNames!()