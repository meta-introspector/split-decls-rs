// Generated macro for impl_220 (impl)
macro_rules! Depcrateimpl_220 {
() => {
// Module: crate
// Provides: {"impl_220"}
// Dependencies: {}
# [cfg (all (feature = "der" , feature = "digest"))] impl < C > From < SignatureWithOid < C > > for der :: Signature < C > where C : EcdsaCurve , der :: MaxSize < C > : ArraySize , < FieldBytesSize < C > as Add > :: Output : Add < der :: MaxOverhead > + ArraySize , { fn from (sig : SignatureWithOid < C >) -> der :: Signature < C > { sig . to_der () } }
};
}
