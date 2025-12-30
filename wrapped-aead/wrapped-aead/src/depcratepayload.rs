// Generated macro for Payload (struct)
macro_rules! DepcratePayload {
() => {
// Module: crate
// Provides: {"Payload"}
// Dependencies: {}
# [doc = " AEAD payloads (message + AAD)."] # [doc = ""] # [doc = " Combination of a message (plaintext or ciphertext) and"] # [doc = " \"additional associated data\" (AAD) to be authenticated (in cleartext)"] # [doc = " along with the message."] # [doc = ""] # [doc = " If you don't care about AAD, you can pass a `&[u8]` as the payload to"] # [doc = " `encrypt`/`decrypt` and it will automatically be coerced to this type."] # [derive (Debug)] pub struct Payload < 'msg , 'aad > { # [doc = " Message to be encrypted/decrypted"] pub msg : & 'msg [u8] , # [doc = " Optional \"additional associated data\" to authenticate along with"] # [doc = " this message. If AAD is provided at the time the message is encrypted,"] # [doc = " the same AAD *MUST* be provided at the time the message is decrypted,"] # [doc = " or decryption will fail."] pub aad : & 'aad [u8] , }
};
}
