// Generated macro for impl_539 (impl)
macro_rules! Depcrate_vectorimpl_539 {
() => {
// Module: crate::vector
// Provides: {"impl_539"}
// Dependencies: {}
impl < A : Clone > From < Vec < A > > for Vector < A > { # [doc = " Create a vector from a [`std::vec::Vec`][vec]."] # [doc = ""] # [doc = " Time: O(n)"] # [doc = ""] # [doc = " [vec]: https://doc.rust-lang.org/std/vec/struct.Vec.html"] fn from (vec : Vec < A >) -> Self { vec . into_iter () . collect () } }
};
}
