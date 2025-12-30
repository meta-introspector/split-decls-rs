// Generated macro for impl_251 (impl)
macro_rules! Depcrate_base_opsimpl_251 {
() => {
// Module: crate::base::ops
// Provides: {"impl_251"}
// Dependencies: {}
impl < T , R : Dim , C : Dim , S > Matrix < T , R , C , S > where T : Scalar + ClosedNeg , S : StorageMut < T , R , C > , { # [doc = " Negates `self` in-place."] # [inline] pub fn neg_mut (& mut self) { for e in self . iter_mut () { * e = - e . clone () } } }
};
}
