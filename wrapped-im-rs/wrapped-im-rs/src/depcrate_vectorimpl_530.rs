// Generated macro for impl_530 (impl)
macro_rules! Depcrate_vectorimpl_530 {
() => {
// Module: crate::vector
// Provides: {"impl_530"}
// Dependencies: {}
impl < 'a , A : Clone > Add for & 'a Vector < A > { type Output = Vector < A > ; # [doc = " Concatenate two vectors."] # [doc = ""] # [doc = " Time: O(log n)"] fn add (self , other : Self) -> Self :: Output { let mut out = self . clone () ; out . append (other . clone ()) ; out } }
};
}
