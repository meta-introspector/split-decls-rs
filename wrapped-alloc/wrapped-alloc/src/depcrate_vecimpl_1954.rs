// Generated macro for impl_1954 (impl)
macro_rules! Depcrate_vecimpl_1954 {
() => {
// Module: crate::vec
// Provides: {"impl_1954"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < T , A : Allocator > IntoIterator for Vec < T , A > { type Item = T ; type IntoIter = IntoIter < T , A > ; # [doc = " Creates a consuming iterator, that is, one that moves each value out of"] # [doc = " the vector (from start to end). The vector cannot be used after calling"] # [doc = " this."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " let v = vec![\"a\".to_string(), \"b\".to_string()];"] # [doc = " let mut v_iter = v.into_iter();"] # [doc = ""] # [doc = " let first_element: Option<String> = v_iter.next();"] # [doc = ""] # [doc = " assert_eq!(first_element, Some(\"a\".to_string()));"] # [doc = " assert_eq!(v_iter.next(), Some(\"b\".to_string()));"] # [doc = " assert_eq!(v_iter.next(), None);"] # [doc = " ```"] # [inline] fn into_iter (self) -> Self :: IntoIter { unsafe { let me = ManuallyDrop :: new (self) ; let alloc = ManuallyDrop :: new (ptr :: read (me . allocator ())) ; let buf = me . buf . non_null () ; let begin = buf . as_ptr () ; let end = if T :: IS_ZST { begin . wrapping_byte_add (me . len ()) } else { begin . add (me . len ()) as * const T } ; let cap = me . buf . capacity () ; IntoIter { buf , phantom : PhantomData , cap , alloc , ptr : buf , end } } } }
};
}
