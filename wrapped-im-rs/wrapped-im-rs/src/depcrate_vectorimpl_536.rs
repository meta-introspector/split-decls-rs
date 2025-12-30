// Generated macro for impl_536 (impl)
macro_rules! Depcrate_vectorimpl_536 {
() => {
// Module: crate::vector
// Provides: {"impl_536"}
// Dependencies: {}
impl < A : Clone > FromIterator < A > for Vector < A > { # [doc = " Create a vector from an iterator."] # [doc = ""] # [doc = " Time: O(n)"] fn from_iter < I > (iter : I) -> Self where I : IntoIterator < Item = A > , { let mut seq = Self :: new () ; for item in iter { seq . push_back (item) } seq } }
};
}
