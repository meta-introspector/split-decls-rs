macro_rules! deps {
    () => {
        DashMap!();
        OwningIter!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        impl < K : Eq + Hash , V > OwningIter < K , V > { pub (crate) fn new < S > (map : DashMap < K , V , S >) -> Self { Self { shards : map . shards . into_vec () . into_iter () , current : None , } } }
    };
}

impl_3!();