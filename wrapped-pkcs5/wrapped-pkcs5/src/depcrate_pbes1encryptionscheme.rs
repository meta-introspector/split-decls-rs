// Generated macro for EncryptionScheme (enum)
macro_rules! Depcrate_pbes1EncryptionScheme {
() => {
// Module: crate::pbes1
// Provides: {"EncryptionScheme"}
// Dependencies: {}
# [doc = " Password-Based Encryption Scheme 1 ciphersuites as defined in [RFC 8018 Appendix A.3]."] # [doc = ""] # [doc = " [RFC 8018 Appendix A.3]: https://tools.ietf.org/html/rfc8018#appendix-A.3"] # [derive (Copy , Clone , Debug , Eq , PartialEq)] pub enum EncryptionScheme { # [doc = " `pbeWithMD2AndDES-CBC`"] PbeWithMd2AndDesCbc , # [doc = " `pbeWithMD2AndRC2-CBC`"] PbeWithMd2AndRc2Cbc , # [doc = " `pbeWithMD5AndDES-CBC`"] PbeWithMd5AndDesCbc , # [doc = " `pbeWithMD5AndRC2-CBC`"] PbeWithMd5AndRc2Cbc , # [doc = " `pbeWithSHA1AndDES-CBC`"] PbeWithSha1AndDesCbc , # [doc = " `pbeWithSHA1AndRC2-CBC`"] PbeWithSha1AndRc2Cbc , }
};
}
