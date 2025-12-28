macro_rules! deps {
    () => {
        FuturesUnordered!();
    };
}

macro_rules! impl_886 {
    () => {
        deps!();
        impl < Fut > Extend < Fut > for FuturesUnordered < Fut > { fn extend < I > (& mut self , iter : I) where I : IntoIterator < Item = Fut > , { for item in iter { self . push (item) ; } } }
    };
}

impl_886!();