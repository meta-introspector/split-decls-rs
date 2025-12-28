macro_rules! deps {
    () => {
        Either!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl < L , R , A > Extend < A > for Either < L , R > where L : Extend < A > , R : Extend < A > , { fn extend < T > (& mut self , iter : T) where T : IntoIterator < Item = A > , { for_both ! (self , inner => inner . extend (iter)) } }
    };
}

impl_17!()