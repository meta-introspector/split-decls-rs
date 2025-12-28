macro_rules! deps {
    () => {
        Vec!();
        Allocator!();
    };
}

macro_rules! impl_162 {
    () => {
        deps!();
        # [doc = " The hash of a vector is the same as that of the corresponding slice,"] # [doc = " as required by the `core::borrow::Borrow` implementation."] # [doc = ""] # [doc = " ```"] # [doc = " use std::hash::BuildHasher;"] # [doc = ""] # [doc = " use allocator_api2::{vec, vec::Vec};"] # [doc = ""] # [doc = " let b = std::hash::RandomState::new();"] # [doc = " let v: Vec<u8> = vec![0xa8, 0x3c, 0x09];"] # [doc = " let s: &[u8] = &[0xa8, 0x3c, 0x09];"] # [doc = " assert_eq!(b.hash_one(v), b.hash_one(s));"] # [doc = " ```"] impl < T : Hash , A : Allocator > Hash for Vec < T , A > { # [inline (always)] fn hash < H : Hasher > (& self , state : & mut H) { Hash :: hash (& * * self , state) } }
    };
}

impl_162!()