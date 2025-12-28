macro_rules! deps {
    () => {
        RawDrain!();
    };
}

macro_rules! impl_104 {
    () => {
        deps!();
        impl < T , A : Allocator > FusedIterator for RawDrain < '_ , T , A > { }
    };
}

impl_104!();