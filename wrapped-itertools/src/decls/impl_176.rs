macro_rules! deps {
    () => {
        LazyBuffer!();
        PoolIndex!();
    };
}

macro_rules! impl_176 {
    () => {
        deps!();
        impl < T > PoolIndex < T > for Box < [usize] > { type Item = Vec < T > ; fn extract_item < I : Iterator < Item = T > > (& self , pool : & LazyBuffer < I >) -> Vec < T > where T : Clone , { pool . get_at (self) } }
    };
}

impl_176!()