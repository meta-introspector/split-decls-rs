macro_rules! deps {
    () => {
        LabeledSample!();
        Label!();
        Float!();
        Iter!();
    };
}

macro_rules! impl_360 {
    () => {
        deps!();
        impl < 'a , A > IntoIterator for & LabeledSample < 'a , A > where A : Float , { type Item = (A , Label) ; type IntoIter = Iter < 'a , A > ; fn into_iter (self) -> Iter < 'a , A > { self . iter () } }
    };
}

impl_360!()