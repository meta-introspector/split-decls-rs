macro_rules! deps {
    () => {
        Splice!();
        Allocator!();
    };
}

macro_rules! impl_104 {
    () => {
        deps!();
        impl < I : Iterator , A : Allocator > ExactSizeIterator for Splice < '_ , I , A > { }
    };
}

impl_104!()