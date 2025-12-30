// Generated macro for validate_ec_evp_key (function)
macro_rules! Depcrate_ecvalidate_ec_evp_key {
() => {
// Module: crate::ec
// Provides: {"validate_ec_evp_key"}
// Dependencies: {}
# [inline] pub (crate) fn validate_ec_evp_key (evp_pkey : & ConstPointer < EVP_PKEY > , expected_curve_nid : i32 ,) -> Result < () , KeyRejected > { let ec_key = evp_pkey . project_const_lifetime (unsafe { | evp_pkey | EVP_PKEY_get0_EC_KEY (* * evp_pkey) }) ? ; verify_ec_key_nid (& ec_key , expected_curve_nid) ? ; # [cfg (not (feature = "fips"))] if 1 != unsafe { EC_KEY_check_key (* ec_key) } { return Err (KeyRejected :: inconsistent_components ()) ; } # [cfg (feature = "fips")] if 1 != indicator_check ! (unsafe { EC_KEY_check_fips (* ec_key) }) { return Err (KeyRejected :: inconsistent_components ()) ; } Ok (()) }
};
}
