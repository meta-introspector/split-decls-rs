// Generated macro for ec_group_from_nid (function)
macro_rules! Depcrate_ecec_group_from_nid {
() => {
// Module: crate::ec
// Provides: {"ec_group_from_nid"}
// Dependencies: {}
# [inline] # [allow (non_upper_case_globals)] pub (crate) fn ec_group_from_nid (nid : i32) -> Result < ConstPointer < 'static , EC_GROUP > , Unspecified > { Ok (unsafe { ConstPointer :: new_static (match nid { NID_secp224r1 => EC_group_p224 () , NID_X9_62_prime256v1 => EC_group_p256 () , NID_secp384r1 => EC_group_p384 () , NID_secp521r1 => EC_group_p521 () , NID_secp256k1 => EC_group_secp256k1 () , _ => { null () } }) ? }) }
};
}
