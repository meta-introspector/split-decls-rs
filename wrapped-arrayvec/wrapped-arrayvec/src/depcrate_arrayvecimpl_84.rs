// Generated macro for impl_84 (impl)
macro_rules! Depcrate_arrayvecimpl_84 {
() => {
// Module: crate::arrayvec
// Provides: {"impl_84"}
// Dependencies: {}
# [doc = " Iterate the `ArrayVec` with mutable references to each element."] # [doc = ""] # [doc = " ```"] # [doc = " use arrayvec::ArrayVec;"] # [doc = ""] # [doc = " let mut array = ArrayVec::from([1, 2, 3]);"] # [doc = ""] # [doc = " for elt in &mut array {"] # [doc = "     // ..."] # [doc = " }"] # [doc = " ```"] impl < 'a , T : 'a , const CAP : usize > IntoIterator for & 'a mut ArrayVec < T , CAP > { type Item = & 'a mut T ; type IntoIter = slice :: IterMut < 'a , T > ; fn into_iter (self) -> Self :: IntoIter { self . iter_mut () } }
};
}
