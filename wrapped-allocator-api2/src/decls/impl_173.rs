macro_rules! deps {
    () => {
        Allocator!();
        Vec!();
    };
}

macro_rules! impl_173 {
    () => {
        deps!();
        impl < T : Eq , A : Allocator > Eq for Vec < T , A > { }
    };
}

impl_173!();