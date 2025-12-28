macro_rules! deps {
    () => {
        Vec!();
        Allocator!();
    };
}

macro_rules! macro_136 {
    () => {
        deps!();
        __impl_slice_eq1 ! { [A : Allocator] Vec < T , A >, & mut [U] }
    };
}

macro_136!();