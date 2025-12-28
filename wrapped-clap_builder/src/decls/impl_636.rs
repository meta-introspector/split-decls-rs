macro_rules! deps {
    () => {
        FlatSet!();
    };
}

macro_rules! impl_636 {
    () => {
        deps!();
        impl < T : PartialEq + Eq > FromIterator < T > for FlatSet < T > { fn from_iter < I : IntoIterator < Item = T > > (iter : I) -> Self { let mut set = Self :: new () ; for value in iter { set . insert (value) ; } set } }
    };
}

impl_636!()