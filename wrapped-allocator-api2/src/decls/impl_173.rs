macro_rules! deps {
    () => {
        Vec!();
        Allocator!();
    };
}

macro_rules! impl_173 {
    () => {
        deps!();
        impl < T : Eq , A : Allocator > Eq for Vec < T , A > { }
    };
}

impl_173!()