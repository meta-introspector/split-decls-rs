// Generated macro for verify_ec_key_nid (function)
macro_rules! Depcrate_ecverify_ec_key_nid {
() => {
// Module: crate::ec
// Provides: {"verify_ec_key_nid"}
// Dependencies: {}
fn verify_ec_key_nid (ec_key : & ConstPointer < EC_KEY > , expected_curve_nid : i32 ,) -> Result < () , KeyRejected > { let ec_group = ec_key . project_const_lifetime (unsafe { | ec_key | EC_KEY_get0_group (* * ec_key) }) ? ; let key_nid = unsafe { EC_GROUP_get_curve_name (* ec_group) } ; if key_nid != expected_curve_nid { return Err (KeyRejected :: wrong_algorithm ()) ; } Ok (()) }
};
}
