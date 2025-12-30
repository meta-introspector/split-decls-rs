// Generated macro for derive_key_utf8 (function)
macro_rules! Depcrate_kdfderive_key_utf8 {
() => {
// Module: crate::kdf
// Provides: {"derive_key_utf8"}
// Dependencies: {}
# [doc = " Derives `key` of type `id` from `pass` and `salt` with length `key_len` using `rounds`"] # [doc = " iterations of the algorithm"] # [doc = " `pass` must be a utf8 string."] # [doc = " ```rust"] # [doc = " let key = pkcs12::kdf::derive_key_utf8::<sha2::Sha256>(\"top-secret\", &[0x1, 0x2, 0x3, 0x4],"] # [doc = "     pkcs12::kdf::Pkcs12KeyType::EncryptionKey, 1000, 32);"] # [doc = " ```"] pub fn derive_key_utf8 < D > (password : & str , salt : & [u8] , id : Pkcs12KeyType , rounds : i32 , key_len : usize ,) -> der :: Result < Vec < u8 > > where D : Digest + FixedOutputReset + BlockSizeUser , { let password_bmp = BmpString :: from_utf8 (password) ? ; Ok (derive_key_bmp :: < D > (password_bmp , salt , id , rounds , key_len)) }
};
}
