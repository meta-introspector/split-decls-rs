macro_rules! deps {
    () => {
        Drain!();
    };
}

macro_rules! impl_523 {
    () => {
        deps!();
        impl < T , A : Allocator > ExactSizeIterator for Drain < '_ , T , A > { fn len (& self) -> usize { self . inner . len () } }
    };
}

impl_523!();