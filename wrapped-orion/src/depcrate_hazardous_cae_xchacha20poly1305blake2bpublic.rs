// Generated macro for public (module)
macro_rules! Depcrate_hazardous_cae_xchacha20poly1305blake2bpublic {
() => {
// Module: crate::hazardous::cae::xchacha20poly1305blake2b
// Provides: {"public"}
// Dependencies: {}
# [cfg (test)] # [cfg (feature = "safe_api")] mod public { use super :: * ; use crate :: test_framework :: aead_interface :: { test_diff_params_err , AeadTestRunner } ; # [quickcheck] # [cfg (feature = "safe_api")] fn prop_aead_interface (input : Vec < u8 > , ad : Vec < u8 >) -> bool { let secret_key = SecretKey :: generate () ; let nonce = Nonce :: generate () ; AeadTestRunner (seal , open , secret_key , nonce , & input , None , TAG_SIZE , & ad) ; test_diff_params_err (& seal , & open , & input , TAG_SIZE) ; true } }
};
}
