// Generated macro for impl_37 (impl)
macro_rules! Depcrate_derimpl_37 {
() => {
// Module: crate::der
// Provides: {"impl_37"}
// Dependencies: {}
impl < C > From < crate :: Signature < C > > for Signature < C > where C : EcdsaCurve , MaxSize < C > : ArraySize , < FieldBytesSize < C > as Add > :: Output : Add < MaxOverhead > + ArraySize , { fn from (sig : crate :: Signature < C >) -> Signature < C > { sig . to_der () } }
};
}
