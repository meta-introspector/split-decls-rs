macro_rules! deps {
    () => {
        Allocator!();
        Vec!();
    };
}

macro_rules! macro_136 {
    () => {
        deps!();
        __impl_slice_eq1 ! { [A : Allocator] Vec < T , A >, & mut [U] }
    };
}

macro_136!()