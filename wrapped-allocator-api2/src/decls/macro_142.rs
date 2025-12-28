macro_rules! deps {
    () => {
        Vec!();
        Allocator!();
    };
}

macro_rules! macro_142 {
    () => {
        deps!();
        __impl_slice_eq1 ! { [A : Allocator , const N : usize] Vec < T , A >, [U ; N] }
    };
}

macro_142!()