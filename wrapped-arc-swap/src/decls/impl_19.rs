macro_rules! deps {
    () => {
        MapGuard!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        impl < G , F , T , R > Deref for MapGuard < G , F , T , R > where G : Deref < Target = T > , F : Fn (& T) -> & R , { type Target = R ; fn deref (& self) -> & R { (self . projection) (& self . guard) } }
    };
}

impl_19!()