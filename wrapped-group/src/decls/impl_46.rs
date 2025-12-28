macro_rules! deps {
    () => {
        GroupOpsOwned!();
        GroupOps!();
    };
}

macro_rules! impl_46 {
    () => {
        deps!();
        impl < T , Rhs , Output > GroupOpsOwned < Rhs , Output > for T where T : for < 'r > GroupOps < & 'r Rhs , Output > { }
    };
}

impl_46!()