// Generated macro for verify (function)
macro_rules! Depcrate_hazardous_kdf_scryptverify {
() => {
// Module: crate::hazardous::kdf::scrypt
// Provides: {"verify"}
// Dependencies: {}
# [allow (clippy :: too_many_arguments)] # [must_use = "SECURITY WARNING: Ignoring a Result can have real security implications."] # [doc = " Verify an scrypt derived key in constant time."] pub fn verify (expected : & [u8] , password : & [u8] , salt : & [u8] , n : u32 , r : u32 , p : u32 , dst_out : & mut [u8] ,) -> Result < () , UnknownCryptoError > { derive_key (password , salt , n , r , p , dst_out) ? ; util :: secure_cmp (dst_out , expected) }
};
}
