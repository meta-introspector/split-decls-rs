// Generated macro for impl_45 (impl)
macro_rules! Depcrate_adaptors_coalesceimpl_45 {
() => {
// Module: crate::adaptors::coalesce
// Provides: {"impl_45"}
// Dependencies: {}
impl < I , F , C > Clone for CoalesceBy < I , F , C > where I : Clone + Iterator , F : Clone , C : CountItem < I :: Item > , C :: CItem : Clone , { clone_fields ! (last , iter , f) ; }
};
}
