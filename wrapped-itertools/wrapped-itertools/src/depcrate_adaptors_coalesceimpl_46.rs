// Generated macro for impl_46 (impl)
macro_rules! Depcrate_adaptors_coalesceimpl_46 {
() => {
// Module: crate::adaptors::coalesce
// Provides: {"impl_46"}
// Dependencies: {}
impl < I , F , C > fmt :: Debug for CoalesceBy < I , F , C > where I : Iterator + fmt :: Debug , C : CountItem < I :: Item > , C :: CItem : fmt :: Debug , { debug_fmt_fields ! (CoalesceBy , iter , last) ; }
};
}
