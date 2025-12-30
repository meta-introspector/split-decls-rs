// Generated macro for UnparsedPublicKey (struct)
macro_rules! Depcrate_signatureUnparsedPublicKey {
() => {
// Module: crate::signature
// Provides: {"UnparsedPublicKey"}
// Dependencies: {}
# [doc = " An unparsed, possibly malformed, public key for signature verification."] # [derive (Clone , Copy)] pub struct UnparsedPublicKey < B > { algorithm : & 'static dyn VerificationAlgorithm , bytes : B , }
};
}
