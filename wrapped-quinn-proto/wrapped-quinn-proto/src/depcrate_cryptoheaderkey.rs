// Generated macro for HeaderKey (trait)
macro_rules! Depcrate_cryptoHeaderKey {
() => {
// Module: crate::crypto
// Provides: {"HeaderKey"}
// Dependencies: {}
# [doc = " Keys used to protect packet headers"] pub trait HeaderKey : Send + Sync { # [doc = " Decrypt the given packet's header"] fn decrypt (& self , pn_offset : usize , packet : & mut [u8]) ; # [doc = " Encrypt the given packet's header"] fn encrypt (& self , pn_offset : usize , packet : & mut [u8]) ; # [doc = " The sample size used for this key's algorithm"] fn sample_size (& self) -> usize ; }
};
}
