// Generated macro for impl_28 (impl)
macro_rules! Depcrate_snapshot_signatureimpl_28 {
() => {
// Module: crate::snapshot::signature
// Provides: {"impl_28"}
// Dependencies: {}
impl < 'a > From < gix_actor :: SignatureRef < 'a > > for Signature < 'a > { fn from (s : gix_actor :: SignatureRef < 'a >) -> Self { Signature { name : s . name . into () , email : s . email . into () , time : s . time . parse () . unwrap_or_default () , } } }
};
}
