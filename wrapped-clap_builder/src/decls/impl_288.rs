macro_rules! deps {
    () => {
        PossibleValuesParser!();
        PossibleValue!();
    };
}

macro_rules! impl_288 {
    () => {
        deps!();
        impl < I , T > From < I > for PossibleValuesParser where I : IntoIterator < Item = T > , T : Into < super :: PossibleValue > , { fn from (values : I) -> Self { Self (values . into_iter () . map (| t | t . into ()) . collect ()) } }
    };
}

impl_288!();