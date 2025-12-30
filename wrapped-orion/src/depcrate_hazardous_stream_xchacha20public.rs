// Generated macro for public (module)
macro_rules! Depcrate_hazardous_stream_xchacha20public {
() => {
// Module: crate::hazardous::stream::xchacha20
// Provides: {"public"}
// Dependencies: {}
# [cfg (test)] # [cfg (feature = "safe_api")] mod public { use super :: * ; mod test_encrypt_decrypt { use super :: * ; use crate :: test_framework :: streamcipher_interface :: * ; impl TestingRandom for Nonce { fn gen () -> Self { Self :: generate () } } # [quickcheck] # [cfg (feature = "safe_api")] fn prop_streamcipher_interface (input : Vec < u8 > , counter : u32) -> bool { let secret_key = SecretKey :: generate () ; let nonce = Nonce :: generate () ; StreamCipherTestRunner (encrypt , decrypt , secret_key , nonce , counter , & input , None) ; test_diff_params_diff_output (& encrypt , & decrypt) ; true } } }
};
}
