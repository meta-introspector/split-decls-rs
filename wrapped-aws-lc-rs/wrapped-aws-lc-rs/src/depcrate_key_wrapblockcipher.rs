// Generated macro for BlockCipher (trait)
macro_rules! Depcrate_key_wrapBlockCipher {
() => {
// Module: crate::key_wrap
// Provides: {"BlockCipher"}
// Dependencies: {}
# [doc = " A key wrap block cipher."] pub trait BlockCipher : 'static + Debug + Sealed { # [doc = " The block cipher identifier."] fn id (& self) -> BlockCipherId ; # [doc = " The key size in bytes to be used with the block cipher."] fn key_len (& self) -> usize ; }
};
}
