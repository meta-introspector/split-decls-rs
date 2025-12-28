macro_rules! deps {
    () => {
        FuturesOrdered!();
    };
}

macro_rules! impl_828 {
    () => {
        deps!();
        impl < Fut : Future > FromIterator < Fut > for FuturesOrdered < Fut > { fn from_iter < T > (iter : T) -> Self where T : IntoIterator < Item = Fut > , { let acc = Self :: new () ; iter . into_iter () . fold (acc , | mut acc , item | { acc . push_back (item) ; acc }) } }
    };
}

impl_828!()