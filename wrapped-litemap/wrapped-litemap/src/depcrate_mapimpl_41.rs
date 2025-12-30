// Generated macro for impl_41 (impl)
macro_rules! Depcrate_mapimpl_41 {
() => {
// Module: crate::map
// Provides: {"impl_41"}
// Dependencies: {}
impl < 'a , V > LiteMap < & 'a str , V , & 'a [(& 'a str , V)] > { # [doc = " Const function to get the value associated with a `&str` key, if it exists."] # [doc = ""] # [doc = " Also returns the index of the value."] # [doc = ""] # [doc = " Note: This function will no longer be needed if const trait behavior is stabilized."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " use litemap::LiteMap;"] # [doc = ""] # [doc = " const MAP: LiteMap<&str, usize, &[(&str, usize)]> ="] # [doc = "     LiteMap::from_sorted_store_unchecked(&["] # [doc = "         (\"abc\", 11),"] # [doc = "         (\"bcd\", 22),"] # [doc = "         (\"cde\", 33),"] # [doc = "         (\"def\", 44),"] # [doc = "         (\"efg\", 55),"] # [doc = "     ]);"] # [doc = ""] # [doc = " assert_eq!(const { MAP.const_get_with_index(\"def\") }, Some((3, &44)));"] # [doc = ""] # [doc = " assert_eq!(const { MAP.const_get_with_index(\"dng\") }, None);"] # [doc = " ```"] pub const fn const_get_with_index (& self , key : & str) -> Option < (usize , & 'a V) > { let mut i = 0 ; let mut j = self . const_len () ; while i < j { let mid = (i + j) / 2 ; # [expect (clippy :: indexing_slicing)] let x = & self . values [mid] ; match const_cmp_bytes (key . as_bytes () , x . 0 . as_bytes ()) { Ordering :: Equal => return Some ((mid , & x . 1)) , Ordering :: Greater => i = mid + 1 , Ordering :: Less => j = mid , } ; } None } }
};
}
