// Generated macro for EvpAead (struct)
macro_rules! Depcrate_backend_aeadEvpAead {
() => {
// Module: crate::backend::aead
// Provides: {"EvpAead"}
// Dependencies: {}
# [cfg (any (CRYPTOGRAPHY_IS_BORINGSSL , CRYPTOGRAPHY_IS_AWSLC))] struct EvpAead { ctx : cryptography_openssl :: aead :: AeadCtx , tag_len : usize , }
};
}
