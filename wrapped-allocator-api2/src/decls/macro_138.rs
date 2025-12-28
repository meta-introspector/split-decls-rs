macro_rules! deps {
    () => {
        Allocator!();
        Vec!();
    };
}

macro_rules! macro_138 {
    () => {
        deps!();
        __impl_slice_eq1 ! { [A : Allocator] & mut [T] , Vec < U , A > }
    };
}

macro_138!()