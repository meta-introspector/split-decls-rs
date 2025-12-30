// Generated macro for impl_1938 (impl)
macro_rules! Depcrate_vecimpl_1938 {
() => {
// Module: crate::vec
// Provides: {"impl_1938"}
// Dependencies: {}
impl < T , A : Allocator , const N : usize > Vec < [T ; N] , A > { # [doc = " Takes a `Vec<[T; N]>` and flattens it into a `Vec<T>`."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if the length of the resulting vector would overflow a `usize`."] # [doc = ""] # [doc = " This is only possible when flattening a vector of arrays of zero-sized"] # [doc = " types, and thus tends to be irrelevant in practice. If"] # [doc = " `size_of::<T>() > 0`, this will never panic."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " let mut vec = vec![[1, 2, 3], [4, 5, 6], [7, 8, 9]];"] # [doc = " assert_eq!(vec.pop(), Some([7, 8, 9]));"] # [doc = ""] # [doc = " let mut flattened = vec.into_flattened();"] # [doc = " assert_eq!(flattened.pop(), Some(6));"] # [doc = " ```"] # [stable (feature = "slice_flatten" , since = "1.80.0")] pub fn into_flattened (self) -> Vec < T , A > { let (ptr , len , cap , alloc) = self . into_raw_parts_with_alloc () ; let (new_len , new_cap) = if T :: IS_ZST { (len . checked_mul (N) . expect ("vec len overflow") , usize :: MAX) } else { unsafe { (len . unchecked_mul (N) , cap . unchecked_mul (N)) } } ; unsafe { Vec :: < T , A > :: from_raw_parts_in (ptr . cast () , new_len , new_cap , alloc) } } }
};
}
