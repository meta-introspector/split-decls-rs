// Generated macro for KeyUsage (struct)
macro_rules! Depcrate_x509_extensionKeyUsage {
() => {
// Module: crate::x509::extension
// Provides: {"KeyUsage"}
// Dependencies: {}
# [doc = " An extension consisting of a list of names of the permitted key usages."] pub struct KeyUsage { critical : bool , digital_signature : bool , non_repudiation : bool , key_encipherment : bool , data_encipherment : bool , key_agreement : bool , key_cert_sign : bool , crl_sign : bool , encipher_only : bool , decipher_only : bool , }
};
}
