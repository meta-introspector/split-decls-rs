macro_rules! deps {
    () => {
        Vec!();
        Allocator!();
    };
}

macro_rules! macro_139 {
    () => {
        deps!();
        __impl_slice_eq1 ! { [A : Allocator] Vec < T , A >, [U] }
    };
}

macro_139!()