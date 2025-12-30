// Generated macro for impl_14 (impl)
macro_rules! Depcrateimpl_14 {
() => {
// Module: crate
// Provides: {"impl_14"}
// Dependencies: {}
impl TryFrom < & [u8] > for Keypair { type Error = SignatureError ; fn try_from (bytes : & [u8]) -> Result < Self , Self :: Error > { let keypair_bytes : & [u8 ; ed25519_dalek :: KEYPAIR_LENGTH] = bytes . try_into () . map_err (| _ | { SignatureError :: from_source (String :: from ("candidate keypair byte array is the wrong length" ,)) }) ? ; ed25519_dalek :: SigningKey :: from_keypair_bytes (keypair_bytes) . map_err (| _ | { SignatureError :: from_source (String :: from ("keypair bytes do not specify same pubkey as derived from their secret key" ,)) }) . map (Self) } }
};
}
