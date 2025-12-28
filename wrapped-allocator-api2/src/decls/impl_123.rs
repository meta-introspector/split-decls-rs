macro_rules! deps {
    () => {
        Allocator!();
        IntoIter!();
    };
}

macro_rules! impl_123 {
    () => {
        deps!();
        impl < T , A : Allocator > AsRef < [T] > for IntoIter < T , A > { fn as_ref (& self) -> & [T] { self . as_slice () } }
    };
}

impl_123!();