// Generated macro for NoProtectionSession (struct)
macro_rules! Depcrate_noprotectionNoProtectionSession {
() => {
// Module: crate::noprotection
// Provides: {"NoProtectionSession"}
// Dependencies: {}
# [doc = " A rustls TLS session which does not perform packet encryption/decryption (for debugging purpose)"] struct NoProtectionSession { inner : Box < dyn crypto :: Session > , }
};
}
