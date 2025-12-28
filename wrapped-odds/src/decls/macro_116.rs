macro_rules! deps {
    () => {
        StrideMut!();
    };
}

macro_rules! macro_116 {
    () => {
        deps!();
        stride_impl ! { struct StrideMut -> &'a mut [A] , as_mut_ptr , * mut A , &'a mut A }
    };
}

macro_116!()