// Generated macro for impl_95 (impl)
macro_rules! Depcrate_signimpl_95 {
() => {
// Module: crate::sign
// Provides: {"impl_95"}
// Dependencies: {}
impl TryFrom < Signature > for InnerSignature { type Error = SigningError ; fn try_from (signature : Signature) -> Result < Self , Self :: Error > { Self :: try_from (& signature) } }
};
}
