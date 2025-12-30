// Generated macro for impl_108 (impl)
macro_rules! Depcrate_arrayvecimpl_108 {
() => {
// Module: crate::arrayvec
// Provides: {"impl_108"}
// Dependencies: {}
# [doc = " Create an `ArrayVec` from an iterator."] # [doc = ""] # [doc = " ***Panics*** if the number of elements in the iterator exceeds the arrayvec's capacity."] impl < T , const CAP : usize > iter :: FromIterator < T > for ArrayVec < T , CAP > { # [doc = " Create an `ArrayVec` from an iterator."] # [doc = ""] # [doc = " ***Panics*** if the number of elements in the iterator exceeds the arrayvec's capacity."] fn from_iter < I : IntoIterator < Item = T > > (iter : I) -> Self { let mut array = ArrayVec :: new () ; array . extend (iter) ; array } }
};
}
