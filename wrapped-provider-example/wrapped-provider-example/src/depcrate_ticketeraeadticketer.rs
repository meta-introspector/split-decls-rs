// Generated macro for AeadTicketer (struct)
macro_rules! Depcrate_ticketerAeadTicketer {
() => {
// Module: crate::ticketer
// Provides: {"AeadTicketer"}
// Dependencies: {}
# [doc = " A [`TicketProducer`] implementation based on random ChaCha20Poly1305 keys."] # [doc = ""] # [doc = " This implementation does not enforce any lifetime constraint."] pub (super) struct AeadTicketer { key : ChaCha20Poly1305 , key_name : [u8 ; 16] , # [doc = " Tracks the largest ciphertext produced by `encrypt`, and"] # [doc = " uses it to early-reject `decrypt` queries that are too long."] # [doc = ""] # [doc = " Accepting excessively long ciphertexts means a \"Partitioning"] # [doc = " Oracle Attack\" (see <https://eprint.iacr.org/2020/1491.pdf>)"] # [doc = " can be more efficient, though also note that these are thought"] # [doc = " to be cryptographically hard if the key is full-entropy (as it"] # [doc = " is here)."] maximum_ciphertext_len : AtomicUsize , }
};
}
