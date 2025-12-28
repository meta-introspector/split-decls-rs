macro_rules! deps {
    () => {
        Pair!();
    };
}

macro_rules! impl_43 {
    () => {
        deps!();
        impl < R : Eq > Eq for Pair < '_ , R > { }
    };
}

impl_43!()