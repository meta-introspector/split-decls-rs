// Generated macro for impl_957 (impl)
macro_rules! Depcrate_collections_vec_deque_iterimpl_957 {
() => {
// Module: crate::collections::vec_deque::iter
// Provides: {"impl_957"}
// Dependencies: {}
impl < 'a , T > Iter < 'a , T > { pub (super) fn new (i1 : slice :: Iter < 'a , T > , i2 : slice :: Iter < 'a , T >) -> Self { Self { i1 , i2 } } # [doc = " Views the underlying data as a pair of subslices of the original data."] # [doc = ""] # [doc = " The slices contain, in order, the contents of the deque not yet yielded"] # [doc = " by the iterator."] # [doc = ""] # [doc = " This has the same lifetime as the original `VecDeque`, and so the"] # [doc = " iterator can continue to be used while this exists."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " #![feature(vec_deque_iter_as_slices)]"] # [doc = ""] # [doc = " use std::collections::VecDeque;"] # [doc = ""] # [doc = " let mut deque = VecDeque::new();"] # [doc = " deque.push_back(0);"] # [doc = " deque.push_back(1);"] # [doc = " deque.push_back(2);"] # [doc = " deque.push_front(10);"] # [doc = " deque.push_front(9);"] # [doc = " deque.push_front(8);"] # [doc = ""] # [doc = " let mut iter = deque.iter();"] # [doc = " iter.next();"] # [doc = " iter.next_back();"] # [doc = ""] # [doc = " assert_eq!(iter.as_slices(), (&[9, 10][..], &[0, 1][..]));"] # [doc = " ```"] # [unstable (feature = "vec_deque_iter_as_slices" , issue = "123947")] pub fn as_slices (& self) -> (& 'a [T] , & 'a [T]) { (self . i1 . as_slice () , self . i2 . as_slice ()) } }
};
}
