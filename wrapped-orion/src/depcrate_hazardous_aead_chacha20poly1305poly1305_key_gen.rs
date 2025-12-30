// Generated macro for poly1305_key_gen (function)
macro_rules! Depcrate_hazardous_aead_chacha20poly1305poly1305_key_gen {
() => {
// Module: crate::hazardous::aead::chacha20poly1305
// Provides: {"poly1305_key_gen"}
// Dependencies: {}
# [doc = " Poly1305 key generation using IETF ChaCha20."] pub (crate) fn poly1305_key_gen (ctx : & mut ChaCha20 , tmp_buffer : & mut Zeroizing < [u8 ; CHACHA_BLOCKSIZE] > ,) -> OneTimeKey { ctx . keystream_block (AUTH_CTR , tmp_buffer . as_mut ()) ; OneTimeKey :: from_slice (& tmp_buffer [.. POLY1305_KEYSIZE]) . unwrap () }
};
}
