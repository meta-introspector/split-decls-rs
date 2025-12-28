macro_rules! deps {
    () => {
        Box!();
        Allocator!();
    };
}

macro_rules! impl_45 {
    () => {
        deps!();
        impl < T , A : Allocator , const N : usize > TryFrom < Box < [T] , A > > for Box < [T ; N] , A > { type Error = Box < [T] , A > ; # [doc = " Attempts to convert a `Box<[T]>` into a `Box<[T; N]>`."] # [doc = ""] # [doc = " The conversion occurs in-place and does not require a"] # [doc = " new memory allocation."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " Returns the old `Box<[T]>` in the `Err` variant if"] # [doc = " `boxed_slice.len()` does not equal `N`."] # [inline (always)] fn try_from (boxed_slice : Box < [T] , A >) -> Result < Self , Self :: Error > { if boxed_slice . len () == N { let (ptr , alloc) = Box :: into_raw_with_allocator (boxed_slice) ; Ok (unsafe { Box :: from_raw_in (ptr as * mut [T ; N] , alloc) }) } else { Err (boxed_slice) } } }
    };
}

impl_45!()