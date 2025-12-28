macro_rules! deps {
    () => {
        Seed!();
        Test!();
    };
}

macro_rules! impl_33 {
    () => {
        deps!();
        impl Test { # [doc = " Create a new \"packed pair\" test from a seed and some given offsets to"] # [doc = " the pair of bytes to use as a predicate in the seed's needle."] # [doc = ""] # [doc = " If a valid test could not be constructed, then None is returned."] # [doc = " (Currently, we take the approach of massaging tests to be valid"] # [doc = " instead of rejecting them outright.)"] fn new (seed : Seed , index1 : usize , index2 : usize , haystack_len : usize , needle_len : usize , fwd : Option < usize > ,) -> Option < Test > { let mut index1 : u8 = index1 . try_into () . unwrap () ; let mut index2 : u8 = index2 . try_into () . unwrap () ; let mut haystack = vec ! [b'@' ; haystack_len] ; let mut needle = vec ! [b'#' ; needle_len] ; needle [0] = seed . first ; needle [index1 as usize] = seed . index1 ; needle [index2 as usize] = seed . index2 ; if let Some (i) = fwd { haystack [i .. i + needle . len ()] . copy_from_slice (& needle) ; } if let Some (i) = crate :: memchr (seed . index1 , & needle) { index1 = u8 :: try_from (i) . unwrap () ; } if let Some (i) = crate :: memchr (seed . index2 , & needle) { index2 = u8 :: try_from (i) . unwrap () ; } Some (Test { haystack , needle , index1 , index2 , fwd }) } }
    };
}

impl_33!();