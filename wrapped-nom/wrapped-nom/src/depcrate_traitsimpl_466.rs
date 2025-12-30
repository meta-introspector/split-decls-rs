// Generated macro for impl_466 (impl)
macro_rules! Depcrate_traitsimpl_466 {
() => {
// Module: crate::traits
// Provides: {"impl_466"}
// Dependencies: {}
impl NomRange < usize > for RangeFull { type Saturating = SaturatingIterator ; type Bounded = Range < usize > ; fn bounds (& self) -> (Bound < usize > , Bound < usize >) { (Bound :: Unbounded , Bound :: Unbounded) } fn contains (& self , item : & usize) -> bool { RangeBounds :: contains (self , item) } fn is_inverted (& self) -> bool { false } fn saturating_iter (& self) -> Self :: Saturating { SaturatingIterator { count : 0 } } fn bounded_iter (& self) -> Self :: Bounded { 0 .. usize :: MAX } }
};
}
