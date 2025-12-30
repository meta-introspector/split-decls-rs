// Generated macro for impl_36 (impl)
macro_rules! Depcrate_derimpl_36 {
() => {
// Module: crate::der
// Provides: {"impl_36"}
// Dependencies: {}
impl < C > FixedTag for Signature < C > where C : EcdsaCurve , MaxSize < C > : ArraySize , < FieldBytesSize < C > as Add > :: Output : Add < MaxOverhead > + ArraySize , { const TAG : Tag = Tag :: Sequence ; }
};
}
