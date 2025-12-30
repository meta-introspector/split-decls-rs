// Generated macro for impl_40 (impl)
macro_rules! Depcrate_derimpl_40 {
() => {
// Module: crate::der
// Provides: {"impl_40"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < C > From < Signature < C > > for Box < [u8] > where C : EcdsaCurve , MaxSize < C > : ArraySize , < FieldBytesSize < C > as Add > :: Output : Add < MaxOverhead > + ArraySize , { fn from (signature : Signature < C >) -> Box < [u8] > { signature . to_vec () . into_boxed_slice () } }
};
}
