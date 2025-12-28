macro_rules! deps {
    () => {
        GroupOps!();
        GroupOpsOwned!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl < T , Rhs , Output > GroupOpsOwned < Rhs , Output > for T where T : for < 'r > GroupOps < & 'r Rhs , Output > { }
    };
}

impl_8!()