// Generated macro for impl_158 (impl)
macro_rules! Depcrate_extensionsimpl_158 {
() => {
// Module: crate::extensions
// Provides: {"impl_158"}
// Dependencies: {}
impl KeyUsage < '_ > { pub fn is_zeroed (& self) -> bool { self . 0 . as_bytes () . iter () . all (| & b | b == 0) } pub fn digital_signature (& self) -> bool { self . 0 . has_bit_set (0) } pub fn content_commitment (& self) -> bool { self . 0 . has_bit_set (1) } pub fn key_encipherment (& self) -> bool { self . 0 . has_bit_set (2) } pub fn data_encipherment (& self) -> bool { self . 0 . has_bit_set (3) } pub fn key_agreement (& self) -> bool { self . 0 . has_bit_set (4) } pub fn key_cert_sign (& self) -> bool { self . 0 . has_bit_set (5) } pub fn crl_sign (& self) -> bool { self . 0 . has_bit_set (6) } pub fn encipher_only (& self) -> bool { self . 0 . has_bit_set (7) } pub fn decipher_only (& self) -> bool { self . 0 . has_bit_set (8) } }
};
}
