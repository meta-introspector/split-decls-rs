// Generated macro for impl_274 (impl)
macro_rules! Depcrateimpl_274 {
() => {
// Module: crate
// Provides: {"impl_274"}
// Dependencies: {}
impl < 'a , P : MlDsaParams > TryFrom < & 'a [u8] > for Signature < P > { type Error = Error ; fn try_from (value : & 'a [u8]) -> Result < Self , Self :: Error > { let enc = EncodedSignature :: < P > :: try_from (value) . map_err (| _ | Error :: new ()) ? ; Self :: decode (& enc) . ok_or (Error :: new ()) } }
};
}
