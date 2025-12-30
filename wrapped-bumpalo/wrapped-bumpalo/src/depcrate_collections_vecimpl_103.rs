// Generated macro for impl_103 (impl)
macro_rules! Depcrate_collections_vecimpl_103 {
() => {
// Module: crate::collections::vec
// Provides: {"impl_103"}
// Dependencies: {}
impl < 'bump , T : 'bump > IntoIterator for Vec < 'bump , T > { type Item = T ; type IntoIter = IntoIter < 'bump , T > ; # [doc = " Creates a consuming iterator, that is, one that moves each value out of"] # [doc = " the vector (from start to end). The vector cannot be used after calling"] # [doc = " this."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use bumpalo::{Bump, collections::Vec};"] # [doc = ""] # [doc = " let b = Bump::new();"] # [doc = ""] # [doc = " let v = bumpalo::vec![in &b; \"a\".to_string(), \"b\".to_string()];"] # [doc = " for s in v.into_iter() {"] # [doc = "     // s has type String, not &String"] # [doc = "     println!(\"{}\", s);"] # [doc = " }"] # [doc = " ```"] # [inline] fn into_iter (mut self) -> IntoIter < 'bump , T > { unsafe { let begin = self . as_mut_ptr () ; let end = if mem :: size_of :: < T > () == 0 { arith_offset (begin as * const i8 , self . len () as isize) as * const T } else { begin . add (self . len ()) as * const T } ; mem :: forget (self) ; IntoIter { phantom : PhantomData , ptr : begin , end , } } } }
};
}
