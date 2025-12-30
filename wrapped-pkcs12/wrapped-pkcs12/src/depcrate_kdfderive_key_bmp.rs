// Generated macro for derive_key_bmp (function)
macro_rules! Depcrate_kdfderive_key_bmp {
() => {
// Module: crate::kdf
// Provides: {"derive_key_bmp"}
// Dependencies: {}
# [doc = " Derive"] pub fn derive_key_bmp < D > (password : BmpString , salt : & [u8] , id : Pkcs12KeyType , rounds : i32 , key_len : usize ,) -> Vec < u8 > where D : Digest + FixedOutputReset + BlockSizeUser , { let mut password = Zeroizing :: new (Vec :: from (password . into_bytes ())) ; password . extend ([0u8 ; 2]) ; derive_key :: < D > (& password , salt , id , rounds , key_len) }
};
}
