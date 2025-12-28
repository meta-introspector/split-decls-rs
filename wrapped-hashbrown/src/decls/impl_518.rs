macro_rules! deps {
    () => {
        IntoIter!();
    };
}

macro_rules! impl_518 {
    () => {
        deps!();
        impl < T , A > ExactSizeIterator for IntoIter < T , A > where A : Allocator , { fn len (& self) -> usize { self . inner . len () } }
    };
}

impl_518!();