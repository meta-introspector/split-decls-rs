// Generated macro for impl_529 (impl)
macro_rules! Depcrate_vectorimpl_529 {
() => {
// Module: crate::vector
// Provides: {"impl_529"}
// Dependencies: {}
impl < A : Clone > Add for Vector < A > { type Output = Vector < A > ; # [doc = " Concatenate two vectors."] # [doc = ""] # [doc = " Time: O(log n)"] fn add (mut self , other : Self) -> Self :: Output { self . append (other) ; self } }
};
}
