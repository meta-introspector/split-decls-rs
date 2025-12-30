// Generated macro for AuthenticatedSafe (type)
macro_rules! Depcrate_authenticated_safeAuthenticatedSafe {
() => {
// Module: crate::authenticated_safe
// Provides: {"AuthenticatedSafe"}
// Dependencies: {}
# [doc = " The `AuthenticatedSafe` type is defined in [RFC 7292 Section 4.1]."] # [doc = ""] # [doc = " ```text"] # [doc = " AuthenticatedSafe ::= SEQUENCE OF ContentInfo"] # [doc = "        -- Data if unencrypted"] # [doc = "        -- EncryptedData if password-encrypted"] # [doc = "        -- EnvelopedData if public key-encrypted"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 7292 Section 4.1]: https://www.rfc-editor.org/rfc/rfc7292#section-4.1"] pub type AuthenticatedSafe < 'a > = Vec < ContentInfo > ;
};
}
