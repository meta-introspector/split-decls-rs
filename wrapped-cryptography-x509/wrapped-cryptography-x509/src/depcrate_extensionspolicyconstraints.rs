// Generated macro for PolicyConstraints (struct)
macro_rules! Depcrate_extensionsPolicyConstraints {
() => {
// Module: crate::extensions
// Provides: {"PolicyConstraints"}
// Dependencies: {}
# [derive (asn1 :: Asn1Read , asn1 :: Asn1Write)] pub struct PolicyConstraints { # [implicit (0)] pub require_explicit_policy : Option < u64 > , # [implicit (1)] pub inhibit_policy_mapping : Option < u64 > , }
};
}
