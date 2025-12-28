macro_rules! deps {
    () => {
        DashMap!();
    };
}

macro_rules! impl_171 {
    () => {
        deps!();
        impl < K : Eq + Hash , V , S : BuildHasher + Clone + Default > FromIterator < (K , V) > for DashMap < K , V , S > { fn from_iter < I : IntoIterator < Item = (K , V) > > (intoiter : I) -> Self { let mut map = DashMap :: default () ; map . extend (intoiter) ; map } }
    };
}

impl_171!()