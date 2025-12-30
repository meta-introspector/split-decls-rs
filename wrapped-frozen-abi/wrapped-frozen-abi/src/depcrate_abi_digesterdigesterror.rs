// Generated macro for DigestError (enum)
macro_rules! Depcrate_abi_digesterDigestError {
() => {
// Module: crate::abi_digester
// Provides: {"DigestError"}
// Dependencies: {}
# [derive (Debug , Error)] pub enum DigestError { # [error ("Option::None is serialized; no ABI digest for Option::Some")] NoneIsSerialized , # [error ("nested error")] Node (Sstr , Box < DigestError >) , # [error ("leaf error")] Leaf (Sstr , Sstr , Box < DigestError >) , # [error ("arithmetic overflow")] ArithmeticOverflow , }
};
}
