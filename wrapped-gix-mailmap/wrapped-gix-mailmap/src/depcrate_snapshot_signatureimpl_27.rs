// Generated macro for impl_27 (impl)
macro_rules! Depcrate_snapshot_signatureimpl_27 {
() => {
// Module: crate::snapshot::signature
// Provides: {"impl_27"}
// Dependencies: {}
impl < 'a > From < Signature < 'a > > for gix_actor :: Signature { fn from (s : Signature < 'a >) -> Self { gix_actor :: Signature { name : s . name . into_owned () , email : s . email . into_owned () , time : s . time , } } }
};
}
