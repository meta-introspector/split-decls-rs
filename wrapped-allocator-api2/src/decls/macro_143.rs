macro_rules! deps {
    () => {
        Allocator!();
        Vec!();
    };
}

macro_rules! macro_143 {
    () => {
        deps!();
        __impl_slice_eq1 ! { [A : Allocator , const N : usize] Vec < T , A >, & [U ; N] }
    };
}

macro_143!()