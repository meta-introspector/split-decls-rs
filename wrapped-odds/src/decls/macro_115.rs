macro_rules! deps {
    () => {
        Stride!();
    };
}

macro_rules! macro_115 {
    () => {
        deps!();
        stride_impl ! { struct Stride -> &'a [A] , as_ptr , * const A , &'a A }
    };
}

macro_115!()