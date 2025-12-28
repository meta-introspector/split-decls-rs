macro_rules! deps {
    () => {
        FuturesOrdered!();
    };
}

macro_rules! impl_830 {
    () => {
        deps!();
        impl < Fut : Future > Extend < Fut > for FuturesOrdered < Fut > { fn extend < I > (& mut self , iter : I) where I : IntoIterator < Item = Fut > , { for item in iter { self . push_back (item) ; } } }
    };
}

impl_830!()