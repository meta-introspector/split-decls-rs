macro_rules! deps {
    () => {
        NavigationTarget!();
    };
}

macro_rules! CallLocations {
    () => {
        deps!();
        # [derive (Default)] struct CallLocations { funcs : FxIndexMap < NavigationTarget , Vec < FileRange > > , }
    };
}

CallLocations!();