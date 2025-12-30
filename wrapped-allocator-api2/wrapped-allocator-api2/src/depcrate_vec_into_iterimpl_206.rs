// Generated macro for impl_206 (impl)
macro_rules! Depcrate_vec_into_iterimpl_206 {
() => {
// Module: crate::vec::into_iter
// Provides: {"impl_206"}
// Dependencies: {}
impl < T , A : Allocator > IntoIter < T , A > { # [doc = " Returns the remaining items of this iterator as a slice."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " let vec = vec!['a', 'b', 'c'];"] # [doc = " let mut into_iter = vec.into_iter();"] # [doc = " assert_eq!(into_iter.as_slice(), &['a', 'b', 'c']);"] # [doc = " let _ = into_iter.next().unwrap();"] # [doc = " assert_eq!(into_iter.as_slice(), &['b', 'c']);"] # [doc = " ```"] pub fn as_slice (& self) -> & [T] { unsafe { slice :: from_raw_parts (self . ptr , self . len ()) } } # [doc = " Returns the remaining items of this iterator as a mutable slice."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " let vec = vec!['a', 'b', 'c'];"] # [doc = " let mut into_iter = vec.into_iter();"] # [doc = " assert_eq!(into_iter.as_slice(), &['a', 'b', 'c']);"] # [doc = " into_iter.as_mut_slice()[2] = 'z';"] # [doc = " assert_eq!(into_iter.next().unwrap(), 'a');"] # [doc = " assert_eq!(into_iter.next().unwrap(), 'b');"] # [doc = " assert_eq!(into_iter.next().unwrap(), 'z');"] # [doc = " ```"] pub fn as_mut_slice (& mut self) -> & mut [T] { unsafe { & mut * self . as_raw_mut_slice () } } # [doc = " Returns a reference to the underlying allocator."] # [inline (always)] pub fn allocator (& self) -> & A { & self . alloc } fn as_raw_mut_slice (& mut self) -> * mut [T] { ptr :: slice_from_raw_parts_mut (self . ptr as * mut T , self . len ()) } }
};
}
