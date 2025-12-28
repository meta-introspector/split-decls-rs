macro_rules! deps {
    () => {
        BlockedIter!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl < 'a , B , T > Copy for BlockedIter < 'a , B , T > { }
    };
}

impl_28!()