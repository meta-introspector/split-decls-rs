// Generated macro for UnparsedPublicKey (struct)
macro_rules! Depcrate_agreementUnparsedPublicKey {
() => {
// Module: crate::agreement
// Provides: {"UnparsedPublicKey"}
// Dependencies: {}
# [doc = " An unparsed, possibly malformed, public key for key agreement."] # [derive (Clone)] pub struct UnparsedPublicKey < B : AsRef < [u8] > > { alg : & 'static Algorithm , bytes : B , }
};
}
