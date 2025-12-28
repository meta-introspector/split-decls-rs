macro_rules! deps {
    () => {
        Vec!();
        Allocator!();
    };
}

macro_rules! impl_153 {
    () => {
        deps!();
        impl < T , A : Allocator , const N : usize > Vec < [T ; N] , A > { # [doc = " Takes a `Vec<[T; N]>` and flattens it into a `Vec<T>`."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if the length of the resulting vector would overflow a `usize`."] # [doc = ""] # [doc = " This is only possible when flattening a vector of arrays of zero-sized"] # [doc = " types, and thus tends to be irrelevant in practice. If"] # [doc = " `size_of::<T>() > 0`, this will never panic."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use allocator_api2::vec;"] # [doc = ""] # [doc = " let mut vec = vec![[1, 2, 3], [4, 5, 6], [7, 8, 9]];"] # [doc = " assert_eq!(vec.pop(), Some([7, 8, 9]));"] # [doc = ""] # [doc = " let mut flattened = vec.into_flattened();"] # [doc = " assert_eq!(flattened.pop(), Some(6));"] # [doc = " ```"] # [inline (always)] pub fn into_flattened (self) -> Vec < T , A > { let (ptr , len , cap , alloc) = self . into_raw_parts_with_alloc () ; let (new_len , new_cap) = if size_of :: < T > () == 0 { (len . checked_mul (N) . expect ("vec len overflow") , usize :: MAX) } else { (len * N , cap * N) } ; unsafe { Vec :: < T , A > :: from_raw_parts_in (ptr . cast () , new_len , new_cap , alloc) } } }
    };
}

impl_153!();