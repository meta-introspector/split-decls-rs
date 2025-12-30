// Generated macro for impl_540 (impl)
macro_rules! Depcrate_vectorimpl_540 {
() => {
// Module: crate::vector
// Provides: {"impl_540"}
// Dependencies: {}
impl < 'a , A : Clone > From < & 'a Vec < A > > for Vector < A > { # [doc = " Create a vector from a [`std::vec::Vec`][vec]."] # [doc = ""] # [doc = " Time: O(n)"] # [doc = ""] # [doc = " [vec]: https://doc.rust-lang.org/std/vec/struct.Vec.html"] fn from (vec : & Vec < A >) -> Self { vec . iter () . cloned () . collect () } }
};
}
