macro_rules! deps {
    () => {
        AttrList!();
        AList!();
    };
}

macro_rules! impl_54 {
    () => {
        deps!();
        impl < A > From < AttrList < A > > for AList < A > { fn from (attr : AttrList < A >) -> Self { attr . into_iter () . flatten () . collect () } }
    };
}

impl_54!();