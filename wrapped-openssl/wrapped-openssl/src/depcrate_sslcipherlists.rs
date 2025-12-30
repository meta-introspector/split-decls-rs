// Generated macro for CipherLists (struct)
macro_rules! Depcrate_sslCipherLists {
() => {
// Module: crate::ssl
// Provides: {"CipherLists"}
// Dependencies: {}
# [doc = " A stack of selected ciphers, and a stack of selected signalling cipher suites"] # [derive (Debug)] pub struct CipherLists { pub suites : Stack < SslCipher > , pub signalling_suites : Stack < SslCipher > , }
};
}
