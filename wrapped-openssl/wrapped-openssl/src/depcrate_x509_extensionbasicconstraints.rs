// Generated macro for BasicConstraints (struct)
macro_rules! Depcrate_x509_extensionBasicConstraints {
() => {
// Module: crate::x509::extension
// Provides: {"BasicConstraints"}
// Dependencies: {}
# [doc = " An extension which indicates whether a certificate is a CA certificate."] pub struct BasicConstraints { critical : bool , ca : bool , pathlen : Option < u32 > , }
};
}
