// Generated macro for ChaCha20 (struct)
macro_rules! Depcrate_chacha20ChaCha20 {
() => {
// Module: crate::chacha20
// Provides: {"ChaCha20"}
// Dependencies: {}
# [doc = " A pure-Rust implementation of the ChaCha20 stream cipher algorithm."] # [doc = " - Produces 256GB of keystream data."] # [doc = " - Zeroes its memory on drop."] # [doc = ""] # [doc = " This code originates from the rust-crypto project:"] # [doc = " <https://github.com/RustCrypto/stream-ciphers>"] # [doc = ""] # [doc = " A clear explanation of the algorithm: <https://loup-vaillant.fr/tutorials/chacha20-design>"] pub struct ChaCha20 { state : [u32 ; STATE_WORDS] , buffer : [u8 ; BUFFER_SIZE] , buffer_pos : usize , }
};
}
