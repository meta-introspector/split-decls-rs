macro_rules! deps {
    () => {
        GroupOps!();
    };
}

macro_rules! impl_44 {
    () => {
        deps!();
        impl < T , Rhs , Output > GroupOps < Rhs , Output > for T where T : Add < Rhs , Output = Output > + Sub < Rhs , Output = Output > + AddAssign < Rhs > + SubAssign < Rhs > { }
    };
}

impl_44!();