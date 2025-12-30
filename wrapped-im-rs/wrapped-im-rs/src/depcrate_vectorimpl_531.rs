// Generated macro for impl_531 (impl)
macro_rules! Depcrate_vectorimpl_531 {
() => {
// Module: crate::vector
// Provides: {"impl_531"}
// Dependencies: {}
impl < A : Clone > Extend < A > for Vector < A > { # [doc = " Add values to the end of a vector by consuming an iterator."] # [doc = ""] # [doc = " Time: O(n)"] fn extend < I > (& mut self , iter : I) where I : IntoIterator < Item = A > , { for item in iter { self . push_back (item) } } }
};
}
