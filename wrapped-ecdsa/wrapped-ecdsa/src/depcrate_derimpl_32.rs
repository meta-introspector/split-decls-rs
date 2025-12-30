// Generated macro for impl_32 (impl)
macro_rules! Depcrate_derimpl_32 {
() => {
// Module: crate::der
// Provides: {"impl_32"}
// Dependencies: {}
impl < C > Clone for Signature < C > where C : EcdsaCurve , MaxSize < C > : ArraySize , < FieldBytesSize < C > as Add > :: Output : Add < MaxOverhead > + ArraySize , { fn clone (& self) -> Self { Self { bytes : self . bytes . clone () , r_range : self . r_range . clone () , s_range : self . s_range . clone () , } } }
};
}
