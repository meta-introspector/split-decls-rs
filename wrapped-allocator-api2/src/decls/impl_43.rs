macro_rules! deps {
    () => {
        Box!();
        Allocator!();
        Vec!();
    };
}

macro_rules! impl_43 {
    () => {
        deps!();
        impl < T , A : Allocator , const N : usize > Box < [T ; N] , A > { # [inline (always)] pub fn slice (b : Self) -> Box < [T] , A > { let (ptr , alloc) = Box :: into_raw_with_allocator (b) ; unsafe { Box :: from_raw_in (ptr , alloc) } } pub fn into_vec (self) -> Vec < T , A > where A : Allocator , { unsafe { let (b , alloc) = Box :: into_raw_with_allocator (self) ; Vec :: from_raw_parts_in (b as * mut T , N , N , alloc) } } }
    };
}

impl_43!();