macro_rules! deps {
    () => {
        Vec!();
        Allocator!();
    };
}

macro_rules! macro_140 {
    () => {
        deps!();
        __impl_slice_eq1 ! { [A : Allocator] [T] , Vec < U , A > }
    };
}

macro_140!();