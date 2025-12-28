macro_rules! deps {
    () => {
        PoolIndex!();
        LazyBuffer!();
    };
}

macro_rules! impl_177 {
    () => {
        deps!();
        impl < T > PoolIndex < T > for Vec < usize > { type Item = Vec < T > ; fn extract_item < I : Iterator < Item = T > > (& self , pool : & LazyBuffer < I >) -> Vec < T > where T : Clone , { pool . get_at (self) } }
    };
}

impl_177!()