// Generated macro for verify_evp_key_nid (function)
macro_rules! Depcrate_ecverify_evp_key_nid {
() => {
// Module: crate::ec
// Provides: {"verify_evp_key_nid"}
// Dependencies: {}
# [inline] # [cfg (not (feature = "fips"))] pub (crate) fn verify_evp_key_nid (evp_pkey : & ConstPointer < EVP_PKEY > , expected_curve_nid : i32 ,) -> Result < () , KeyRejected > { let ec_key = evp_pkey . project_const_lifetime (unsafe { | evp_pkey | EVP_PKEY_get0_EC_KEY (* * evp_pkey) }) ? ; verify_ec_key_nid (& ec_key , expected_curve_nid) ? ; Ok (()) }
};
}
