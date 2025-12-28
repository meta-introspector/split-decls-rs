macro_rules! deps {
    () => {
        Id!();
        FlatMap!();
    };
}

macro_rules! Conflicts {
    () => {
        deps!();
        # [derive (Default , Clone , Debug)] struct Conflicts { potential : FlatMap < Id , Vec < Id > > , }
    };
}

Conflicts!();