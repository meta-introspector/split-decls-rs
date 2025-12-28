macro_rules! deps {
    () => {
        FuturesUnordered!();
    };
}

macro_rules! impl_884 {
    () => {
        deps!();
        impl < Fut > FromIterator < Fut > for FuturesUnordered < Fut > { fn from_iter < I > (iter : I) -> Self where I : IntoIterator < Item = Fut > , { let acc = Self :: new () ; iter . into_iter () . fold (acc , | acc , item | { acc . push (item) ; acc }) } }
    };
}

impl_884!()