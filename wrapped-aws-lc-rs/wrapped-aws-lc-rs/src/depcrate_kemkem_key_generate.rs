// Generated macro for kem_key_generate (function)
macro_rules! Depcrate_kemkem_key_generate {
() => {
// Module: crate::kem
// Provides: {"kem_key_generate"}
// Dependencies: {}
# [inline] fn kem_key_generate (nid : i32) -> Result < LcPtr < EVP_PKEY > , Unspecified > { let params_fn = | ctx | { if 1 == unsafe { EVP_PKEY_CTX_kem_set_params (ctx , nid) } { Ok (()) } else { Err (()) } } ; LcPtr :: < EVP_PKEY > :: generate (EVP_PKEY_KEM , Some (params_fn)) }
};
}
