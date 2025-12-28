macro_rules! deps {
    () => {
        Vec!();
        Allocator!();
        Box!();
    };
}

macro_rules! impl_187 {
    () => {
        deps!();
        # [cfg (not (no_global_oom_handling))] impl < T , A : Allocator > From < Vec < T , A > > for Box < [T] , A > { # [doc = " Convert a vector into a boxed slice."] # [doc = ""] # [doc = " If `v` has excess capacity, its items will be moved into a"] # [doc = " newly-allocated buffer with exactly the right capacity."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use allocator_api2::{vec, boxed::Box};"] # [doc = ""] # [doc = " assert_eq!(Box::from(vec![1, 2, 3]), vec![1, 2, 3].into_boxed_slice());"] # [doc = " ```"] # [doc = ""] # [doc = " Any excess capacity is removed:"] # [doc = " ```"] # [doc = " use allocator_api2::{vec, vec::Vec, boxed::Box};"] # [doc = ""] # [doc = " let mut vec = Vec::with_capacity(10);"] # [doc = " vec.extend([1i32, 2, 3]);"] # [doc = ""] # [doc = " assert_eq!(Box::from(vec), vec![1i32, 2, 3].into_boxed_slice());"] # [doc = " ```"] # [inline (always)] fn from (v : Vec < T , A >) -> Self { v . into_boxed_slice () } }
    };
}

impl_187!();