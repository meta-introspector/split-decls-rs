macro_rules! deps {
    () => {
        RawIntoIter!();
    };
}

macro_rules! impl_96 {
    () => {
        deps!();
        impl < T , A : Allocator > FusedIterator for RawIntoIter < T , A > { }
    };
}

impl_96!()