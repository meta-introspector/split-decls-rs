macro_rules! deps {
    () => {
        AttrList!();
        AList!();
    };
}

macro_rules! impl_55 {
    () => {
        deps!();
        impl < 'a , A > From < & 'a AttrList < A > > for AList < & 'a A > { fn from (attr : & 'a AttrList < A >) -> Self { attr . into_iter () . flatten () . collect () } }
    };
}

impl_55!()