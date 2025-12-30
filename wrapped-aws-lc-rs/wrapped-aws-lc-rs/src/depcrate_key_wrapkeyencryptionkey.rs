// Generated macro for KeyEncryptionKey (struct)
macro_rules! Depcrate_key_wrapKeyEncryptionKey {
() => {
// Module: crate::key_wrap
// Provides: {"KeyEncryptionKey"}
// Dependencies: {}
# [doc = " The key-encryption key used with the selected cipher algorithn to wrap or unwrap a key."] # [doc = ""] # [doc = " Implements the NIST SP 800-38F key wrapping algoirthm."] # [doc = ""] # [doc = " The NIST specification is similar to that of RFC 3394 but with the following caveats:"] # [doc = " * Specifies a maxiumum plaintext length that can be accepted."] # [doc = " * Allows implementations to specify a subset of valid lengths accepted."] # [doc = " * Allows for the usage of other 128-bit block ciphers other than AES."] pub struct KeyEncryptionKey < Cipher : BlockCipher > { cipher : & 'static Cipher , key : Box < [u8] > , }
};
}
