macro_rules! deps {
    () => {
        AList!();
    };
}

macro_rules! impl_51 {
    () => {
        deps!();
        impl < A > FromIterator < A > for AList < A > { fn from_iter < T > (iter : T) -> Self where T : IntoIterator < Item = A > , { Self { elems : iter . into_iter () . collect () , } } }
    };
}

impl_51!()