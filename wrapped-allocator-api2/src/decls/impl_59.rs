macro_rules! deps {
    () => {
        Box!();
        Allocator!();
    };
}

macro_rules! impl_59 {
    () => {
        deps!();
        impl < I : FusedIterator + ? Sized , A : Allocator > FusedIterator for Box < I , A > { }
    };
}

impl_59!()