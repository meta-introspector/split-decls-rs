// Generated macro for impl_37 (impl)
macro_rules! Depcrate_range_set_array_range_setimpl_37 {
() => {
// Module: crate::range_set::array_range_set
// Provides: {"impl_37"}
// Dependencies: {}
impl Clone for ArrayRangeSet { fn clone (& self) -> Self { if self . 0 . is_inline () || self . 0 . len () > ARRAY_RANGE_SET_INLINE_CAPACITY { return Self (self . 0 . clone ()) ; } let mut vec = TinyVec :: new () ; vec . extend_from_slice (self . 0 . as_slice ()) ; Self (vec) } }
};
}
