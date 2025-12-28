macro_rules! deps {
    () => {
        DashMap!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl < K : Eq + Hash + fmt :: Debug , V : fmt :: Debug , S : BuildHasher + Clone > fmt :: Debug for DashMap < K , V , S > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut pmap = f . debug_map () ; for r in self { let (k , v) = r . pair () ; pmap . entry (k , v) ; } pmap . finish () } }
    };
}

impl_22!()