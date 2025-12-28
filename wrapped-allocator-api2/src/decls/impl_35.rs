macro_rules! deps {
    () => {
        Box!();
        Allocator!();
    };
}

macro_rules! impl_35 {
    () => {
        deps!();
        impl < T : ? Sized + Eq , A : Allocator > Eq for Box < T , A > { }
    };
}

impl_35!();