// Generated macro for fail_test (function)
macro_rules! Depcrate_devfail_test {
() => {
// Module: crate::dev
// Provides: {"fail_test"}
// Dependencies: {}
# [doc = " Run AEAD test for the provided failing test vector"] pub fn fail_test < C : AeadInOut + KeyInit > (& TestVector { key , nonce , aad , ciphertext , .. } : & TestVector ,) -> Result < () , & 'static str > { let nonce = nonce . try_into () . expect ("wrong nonce size") ; let cipher = < C as KeyInit > :: new_from_slice (key) . expect ("failed to initialize the cipher") ; let res = cipher . decrypt (nonce , Payload { aad , msg : ciphertext , } ,) ; if res . is_ok () { Err ("decryption must return error") } else { Ok (()) } }
};
}
