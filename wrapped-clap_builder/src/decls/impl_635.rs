macro_rules! deps {
    () => {
        FlatSet!();
    };
}

macro_rules! impl_635 {
    () => {
        deps!();
        impl < T : PartialEq + Eq > Extend < T > for FlatSet < T > { fn extend < I : IntoIterator < Item = T > > (& mut self , iter : I) { for value in iter { self . insert (value) ; } } }
    };
}

impl_635!();