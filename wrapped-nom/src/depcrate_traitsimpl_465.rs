// Generated macro for impl_465 (impl)
macro_rules! Depcrate_traitsimpl_465 {
() => {
// Module: crate::traits
// Provides: {"impl_465"}
// Dependencies: {}
impl NomRange < usize > for RangeToInclusive < usize > { type Saturating = Range < usize > ; type Bounded = Range < usize > ; fn bounds (& self) -> (Bound < usize > , Bound < usize >) { (Bound :: Unbounded , Bound :: Included (self . end)) } fn contains (& self , item : & usize) -> bool { RangeBounds :: contains (self , item) } fn is_inverted (& self) -> bool { false } fn saturating_iter (& self) -> Self :: Saturating { 0 .. self . end } fn bounded_iter (& self) -> Self :: Bounded { 0 .. self . end } }
};
}
