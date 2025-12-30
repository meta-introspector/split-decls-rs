// Generated macro for public (module)
macro_rules! Depcrate_hazardous_aead_chacha20poly1305public {
() => {
// Module: crate::hazardous::aead::chacha20poly1305
// Provides: {"public"}
// Dependencies: {}
# [cfg (test)] # [cfg (feature = "safe_api")] mod public { use super :: * ; use crate :: test_framework :: aead_interface :: { test_diff_params_err , AeadTestRunner } ; # [quickcheck] # [cfg (feature = "safe_api")] fn prop_aead_interface (input : Vec < u8 > , ad : Vec < u8 >) -> bool { let secret_key = SecretKey :: generate () ; let nonce = Nonce :: from_slice (& [0u8 ; chacha20 :: IETF_CHACHA_NONCESIZE]) . unwrap () ; AeadTestRunner (seal , open , secret_key , nonce , & input , None , POLY1305_OUTSIZE , & ad ,) ; test_diff_params_err (& seal , & open , & input , POLY1305_OUTSIZE) ; true } }
};
}
