macro_rules! deps {
    () => {
        Allocator!();
        Box!();
    };
}

macro_rules! impl_66 {
    () => {
        deps!();
        impl < T : ? Sized , A : Allocator > Unpin for Box < T , A > where A : 'static { }
    };
}

impl_66!()