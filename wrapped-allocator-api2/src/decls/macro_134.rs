macro_rules! deps {
    () => {
        Allocator!();
        Vec!();
    };
}

macro_rules! macro_134 {
    () => {
        deps!();
        __impl_slice_eq1 ! { [A1 : Allocator , A2 : Allocator] Vec < T , A1 >, Vec < U , A2 > }
    };
}

macro_134!()