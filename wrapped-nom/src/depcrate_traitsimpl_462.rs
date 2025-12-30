// Generated macro for impl_462 (impl)
macro_rules! Depcrate_traitsimpl_462 {
() => {
// Module: crate::traits
// Provides: {"impl_462"}
// Dependencies: {}
impl NomRange < usize > for RangeInclusive < usize > { type Saturating = Range < usize > ; type Bounded = Range < usize > ; fn bounds (& self) -> (Bound < usize > , Bound < usize >) { (Bound :: Included (* self . start ()) , Bound :: Included (* self . end ())) } fn contains (& self , item : & usize) -> bool { RangeBounds :: contains (self , item) } fn is_inverted (& self) -> bool { ! RangeInclusive :: contains (self , self . start ()) } fn saturating_iter (& self) -> Self :: Saturating { 0 .. * self . end () } fn bounded_iter (& self) -> Self :: Bounded { 0 .. * self . end () } }
};
}
