// Generated macro for impl_1979 (impl)
macro_rules! Depcrate_vecimpl_1979 {
() => {
// Module: crate::vec
// Provides: {"impl_1979"}
// Dependencies: {}
# [stable (feature = "array_try_from_vec" , since = "1.48.0")] impl < T , A : Allocator , const N : usize > TryFrom < Vec < T , A > > for [T ; N] { type Error = Vec < T , A > ; # [doc = " Gets the entire contents of the `Vec<T>` as an array,"] # [doc = " if its size exactly matches that of the requested array."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " assert_eq!(vec![1, 2, 3].try_into(), Ok([1, 2, 3]));"] # [doc = " assert_eq!(<Vec<i32>>::new().try_into(), Ok([]));"] # [doc = " ```"] # [doc = ""] # [doc = " If the length doesn't match, the input comes back in `Err`:"] # [doc = " ```"] # [doc = " let r: Result<[i32; 4], _> = (0..10).collect::<Vec<_>>().try_into();"] # [doc = " assert_eq!(r, Err(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]));"] # [doc = " ```"] # [doc = ""] # [doc = " If you're fine with just getting a prefix of the `Vec<T>`,"] # [doc = " you can call [`.truncate(N)`](Vec::truncate) first."] # [doc = " ```"] # [doc = " let mut v = String::from(\"hello world\").into_bytes();"] # [doc = " v.sort();"] # [doc = " v.truncate(2);"] # [doc = " let [a, b]: [_; 2] = v.try_into().unwrap();"] # [doc = " assert_eq!(a, b' ');"] # [doc = " assert_eq!(b, b'd');"] # [doc = " ```"] fn try_from (mut vec : Vec < T , A >) -> Result < [T ; N] , Vec < T , A > > { if vec . len () != N { return Err (vec) ; } unsafe { vec . set_len (0) } ; let array = unsafe { ptr :: read (vec . as_ptr () as * const [T ; N]) } ; Ok (array) } }
};
}
