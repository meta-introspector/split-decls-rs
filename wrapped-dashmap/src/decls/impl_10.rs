macro_rules! deps {
    () => {
        DashMap!();
        Iter!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl < 'a , K : Eq + Hash + 'a , V : 'a > Iter < 'a , K , V > { pub (crate) fn new < S > (map : & 'a DashMap < K , V , S >) -> Self { Self { shards : map . shards . iter () , current : None , } } }
    };
}

impl_10!()