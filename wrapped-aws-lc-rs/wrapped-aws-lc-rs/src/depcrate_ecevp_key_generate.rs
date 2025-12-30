// Generated macro for evp_key_generate (function)
macro_rules! Depcrate_ecevp_key_generate {
() => {
// Module: crate::ec
// Provides: {"evp_key_generate"}
// Dependencies: {}
# [inline] pub (crate) fn evp_key_generate (nid : c_int) -> Result < LcPtr < EVP_PKEY > , Unspecified > { let params_fn = | ctx | { if 1 == unsafe { EVP_PKEY_CTX_set_ec_paramgen_curve_nid (ctx , nid) } { Ok (()) } else { Err (()) } } ; LcPtr :: < EVP_PKEY > :: generate (EVP_PKEY_EC , Some (params_fn)) }
};
}
