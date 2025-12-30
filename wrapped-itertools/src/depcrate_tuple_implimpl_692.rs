// Generated macro for impl_692 (impl)
macro_rules! Depcrate_tuple_implimpl_692 {
() => {
// Module: crate::tuple_impl
// Provides: {"impl_692"}
// Dependencies: {}
impl < I , T > Tuples < I , T > where I : Iterator < Item = T :: Item > , T : HomogeneousTuple , { # [doc = " Return a buffer with the produced items that was not enough to be grouped in a tuple."] # [doc = ""] # [doc = " ```"] # [doc = " use itertools::Itertools;"] # [doc = ""] # [doc = " let mut iter = (0..5).tuples();"] # [doc = " assert_eq!(Some((0, 1, 2)), iter.next());"] # [doc = " assert_eq!(None, iter.next());"] # [doc = " itertools::assert_equal(vec![3, 4], iter.into_buffer());"] # [doc = " ```"] pub fn into_buffer (self) -> TupleBuffer < T > { TupleBuffer :: new (self . buf) } }
};
}
