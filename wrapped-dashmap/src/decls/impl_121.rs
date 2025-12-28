macro_rules! deps {
    () => {
        DashSet!();
    };
}

macro_rules! impl_121 {
    () => {
        deps!();
        impl < K : Eq + Hash , S : BuildHasher + Clone + Default > FromIterator < K > for DashSet < K , S > { fn from_iter < I : IntoIterator < Item = K > > (iter : I) -> Self { let mut set = DashSet :: default () ; set . extend (iter) ; set } }
    };
}

impl_121!()