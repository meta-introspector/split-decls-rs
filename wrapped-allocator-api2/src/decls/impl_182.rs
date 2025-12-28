macro_rules! deps {
    () => {
        Vec!();
    };
}

macro_rules! impl_182 {
    () => {
        deps!();
        # [cfg (not (no_global_oom_handling))] impl < T : Clone > From < & [T] > for Vec < T > { # [doc = " Allocate a `Vec<T>` and fill it by cloning `s`'s items."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use allocator_api2::{vec, vec::Vec};"] # [doc = ""] # [doc = " assert_eq!(Vec::from(&[1, 2, 3][..]), vec![1, 2, 3]);"] # [doc = " ```"] # [inline (always)] fn from (s : & [T]) -> Vec < T > { let mut vec = Vec :: with_capacity (s . len ()) ; vec . extend_from_slice (s) ; vec } }
    };
}

impl_182!()