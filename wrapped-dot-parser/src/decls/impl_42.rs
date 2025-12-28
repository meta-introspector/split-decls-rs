macro_rules! deps {
    () => {
        AList!();
        AttrList!();
    };
}

macro_rules! impl_42 {
    () => {
        deps!();
        impl < A > FromIterator < AList < A > > for AttrList < A > { fn from_iter < T > (iter : T) -> Self where T : IntoIterator < Item = AList < A > > , { Self { elems : iter . into_iter () . map (| u | u . into_iter () . collect ()) . collect () , } } }
    };
}

impl_42!()