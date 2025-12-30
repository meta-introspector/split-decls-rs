// Generated macro for impl_275 (impl)
macro_rules! Depcrateimpl_275 {
() => {
// Module: crate
// Provides: {"impl_275"}
// Dependencies: {}
impl < P : MlDsaParams > TryInto < EncodedSignature < P > > for Signature < P > { type Error = Error ; fn try_into (self) -> Result < EncodedSignature < P > , Self :: Error > { Ok (self . encode ()) } }
};
}
