macro_rules! deps {
    () => {
        Vec!();
        Allocator!();
    };
}

macro_rules! impl_189 {
    () => {
        deps!();
        impl < T , A : Allocator , const N : usize > TryFrom < Vec < T , A > > for [T ; N] { type Error = Vec < T , A > ; # [doc = " Gets the entire contents of the `Vec<T>` as an array,"] # [doc = " if its size exactly matches that of the requested array."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::convert::TryInto;"] # [doc = ""] # [doc = " use allocator_api2::{vec, vec::Vec};"] # [doc = ""] # [doc = " assert_eq!(vec![1, 2, 3].try_into(), Ok([1, 2, 3]));"] # [doc = " assert_eq!(<Vec<i32>>::new().try_into(), Ok([]));"] # [doc = " ```"] # [doc = ""] # [doc = " If the length doesn't match, the input comes back in `Err`:"] # [doc = " ```"] # [doc = " use std::convert::TryInto;"] # [doc = ""] # [doc = " use allocator_api2::{vec, vec::Vec};"] # [doc = ""] # [doc = " let r: Result<[i32; 4], _> = (0..10).collect::<Vec<_>>().try_into();"] # [doc = " assert_eq!(r, Err(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]));"] # [doc = " ```"] # [doc = ""] # [doc = " If you're fine with just getting a prefix of the `Vec<T>`,"] # [doc = " you can call [`.truncate(N)`](Vec::truncate) first."] # [doc = " ```"] # [doc = " use std::convert::TryInto;"] # [doc = ""] # [doc = " use allocator_api2::vec::Vec;"] # [doc = ""] # [doc = " let mut v = Vec::new();"] # [doc = " v.extend_from_slice(b\"hello world\");"] # [doc = " v.sort();"] # [doc = " v.truncate(2);"] # [doc = " let [a, b]: [_; 2] = v.try_into().unwrap();"] # [doc = " assert_eq!(a, b' ');"] # [doc = " assert_eq!(b, b'd');"] # [doc = " ```"] # [inline (always)] fn try_from (mut vec : Vec < T , A >) -> Result < [T ; N] , Vec < T , A > > { if vec . len () != N { return Err (vec) ; } unsafe { vec . set_len (0) } ; let array = unsafe { ptr :: read (vec . as_ptr () as * const [T ; N]) } ; Ok (array) } }
    };
}

impl_189!();