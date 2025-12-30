// Generated macro for derive_initial_secret (function)
macro_rules! Depcrate_cryptoderive_initial_secret {
() => {
// Module: crate::crypto
// Provides: {"derive_initial_secret"}
// Dependencies: {}
fn derive_initial_secret (secret : & [u8] , version : u32 , out_prk : & mut [u8] ,) -> Result < () > { const INITIAL_SALT_V1 : [u8 ; 20] = [0x38 , 0x76 , 0x2c , 0xf7 , 0xf5 , 0x59 , 0x34 , 0xb3 , 0x4d , 0x17 , 0x9a , 0xe6 , 0xa4 , 0xc8 , 0x0c , 0xad , 0xcc , 0xbb , 0x7f , 0x0a ,] ; let salt = match version { crate :: PROTOCOL_VERSION_V1 => & INITIAL_SALT_V1 , _ => & INITIAL_SALT_V1 , } ; hkdf_extract (Algorithm :: AES128_GCM , out_prk , secret , salt) }
};
}
