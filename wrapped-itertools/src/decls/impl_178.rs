macro_rules! deps {
    () => {
        LazyBuffer!();
        PoolIndex!();
    };
}

macro_rules! impl_178 {
    () => {
        deps!();
        impl < T , const K : usize > PoolIndex < T > for [usize ; K] { type Item = [T ; K] ; fn extract_item < I : Iterator < Item = T > > (& self , pool : & LazyBuffer < I >) -> [T ; K] where T : Clone , { pool . get_array (* self) } }
    };
}

impl_178!();