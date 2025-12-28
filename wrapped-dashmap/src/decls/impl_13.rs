macro_rules! deps {
    () => {
        IterMut!();
        DashMap!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        impl < 'a , K : Eq + Hash + 'a , V : 'a > IterMut < 'a , K , V > { pub (crate) fn new < S > (map : & 'a DashMap < K , V , S >) -> Self { Self { shards : map . shards . iter () , current : None , } } }
    };
}

impl_13!()