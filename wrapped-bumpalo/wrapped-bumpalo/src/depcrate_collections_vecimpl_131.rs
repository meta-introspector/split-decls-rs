// Generated macro for impl_131 (impl)
macro_rules! Depcrate_collections_vecimpl_131 {
() => {
// Module: crate::collections::vec
// Provides: {"impl_131"}
// Dependencies: {}
impl < 'bump , T : 'bump > IntoIter < 'bump , T > { # [doc = " Returns the remaining items of this iterator as a slice."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use bumpalo::{Bump, collections::Vec};"] # [doc = ""] # [doc = " let b = Bump::new();"] # [doc = ""] # [doc = " let vec = bumpalo::vec![in &b; 'a', 'b', 'c'];"] # [doc = " let mut into_iter = vec.into_iter();"] # [doc = " assert_eq!(into_iter.as_slice(), &['a', 'b', 'c']);"] # [doc = " let _ = into_iter.next().unwrap();"] # [doc = " assert_eq!(into_iter.as_slice(), &['b', 'c']);"] # [doc = " ```"] pub fn as_slice (& self) -> & [T] { unsafe { slice :: from_raw_parts (self . ptr , self . len ()) } } # [doc = " Returns the remaining items of this iterator as a mutable slice."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use bumpalo::{Bump, collections::Vec};"] # [doc = ""] # [doc = " let b = Bump::new();"] # [doc = ""] # [doc = " let vec = bumpalo::vec![in &b; 'a', 'b', 'c'];"] # [doc = " let mut into_iter = vec.into_iter();"] # [doc = " assert_eq!(into_iter.as_slice(), &['a', 'b', 'c']);"] # [doc = " into_iter.as_mut_slice()[2] = 'z';"] # [doc = " assert_eq!(into_iter.next().unwrap(), 'a');"] # [doc = " assert_eq!(into_iter.next().unwrap(), 'b');"] # [doc = " assert_eq!(into_iter.next().unwrap(), 'z');"] # [doc = " ```"] pub fn as_mut_slice (& mut self) -> & mut [T] { unsafe { slice :: from_raw_parts_mut (self . ptr as * mut T , self . len ()) } } }
};
}
