macro_rules! deps {
    () => {
        Allocator!();
        Vec!();
    };
}

macro_rules! macro_137 {
    () => {
        deps!();
        __impl_slice_eq1 ! { [A : Allocator] & [T] , Vec < U , A > }
    };
}

macro_137!();