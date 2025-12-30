// Generated macro for Encoding (enum)
macro_rules! Depcrate_encodingEncoding {
() => {
// Module: crate::encoding
// Provides: {"Encoding"}
// Dependencies: {}
# [doc = " Base64 encoding variants."] # [derive (Copy , Clone , Default , Debug , Eq , PartialEq , PartialOrd , Ord)] # [non_exhaustive] pub enum Encoding { # [doc = " \"B64\" encoding: standard Base64 without padding."] # [doc = ""] # [doc = " ```text"] # [doc = " [A-Z]      [a-z]      [0-9]      +     /"] # [doc = " 0x41-0x5a, 0x61-0x7a, 0x30-0x39, 0x2b, 0x2f"] # [doc = " ```"] # [doc = " <https://github.com/P-H-C/phc-string-format/blob/master/phc-sf-spec.md#b64>"] # [default] B64 , # [doc = " bcrypt encoding."] # [doc = ""] # [doc = " ```text"] # [doc = " ./         [A-Z]      [a-z]     [0-9]"] # [doc = " 0x2e-0x2f, 0x41-0x5a, 0x61-0x7a, 0x30-0x39"] # [doc = " ```"] Bcrypt , # [doc = " `crypt(3)` encoding."] # [doc = ""] # [doc = " ```text"] # [doc = " [.-9]      [A-Z]      [a-z]"] # [doc = " 0x2e-0x39, 0x41-0x5a, 0x61-0x7a"] # [doc = " ```"] Crypt , # [doc = " `crypt(3)` Base64 encoding for the following schemes."] # [doc = " - sha1_crypt,"] # [doc = " - sha256_crypt,"] # [doc = " - sha512_crypt,"] # [doc = " - md5_crypt"] # [doc = ""] # [doc = " ```text"] # [doc = " [.-9]      [A-Z]      [a-z]"] # [doc = " 0x2e-0x39, 0x41-0x5a, 0x61-0x7a"] # [doc = " ```"] ShaCrypt , }
};
}
