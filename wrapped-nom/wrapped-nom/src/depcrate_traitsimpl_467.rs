// Generated macro for impl_467 (impl)
macro_rules! Depcrate_traitsimpl_467 {
() => {
// Module: crate::traits
// Provides: {"impl_467"}
// Dependencies: {}
impl NomRange < usize > for usize { type Saturating = Range < usize > ; type Bounded = Range < usize > ; fn bounds (& self) -> (Bound < usize > , Bound < usize >) { (Bound :: Included (* self) , Bound :: Included (* self)) } fn contains (& self , item : & usize) -> bool { self == item } fn is_inverted (& self) -> bool { false } fn saturating_iter (& self) -> Self :: Saturating { 0 .. * self } fn bounded_iter (& self) -> Self :: Bounded { 0 .. * self } }
};
}
