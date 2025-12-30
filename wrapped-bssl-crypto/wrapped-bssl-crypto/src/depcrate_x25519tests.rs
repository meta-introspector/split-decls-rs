// Generated macro for tests (module)
macro_rules! Depcrate_x25519tests {
() => {
// Module: crate::x25519
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] # [allow (clippy :: unwrap_used)] mod tests { use crate :: { test_helpers :: decode_hex , x25519 :: PrivateKey } ; # [test] fn known_vector () { let public_key : [u8 ; 32] = decode_hex ("504a36999f489cd2fdbc08baff3d88fa00569ba986cba22548ffde80f9806829") ; let private_key = PrivateKey (decode_hex ("c8a9d5a91091ad851c668b0736c1c9a02936c0d3ad62670858088047ba057475" ,)) ; let expected_shared_secret : [u8 ; 32] = decode_hex ("436a2c040cf45fea9b29a0cb81b1f41458f863d0d61b453d0a982720d6d61320") ; let shared_secret = private_key . compute_shared_key (& public_key) . unwrap () ; assert_eq ! (expected_shared_secret , shared_secret) ; } # [test] fn all_zero_public_key () { assert ! (PrivateKey :: generate () . 1 . compute_shared_key (& [0u8 ; 32]) . is_none ()) ; } # [test] fn to_public () { let public_key_bytes = decode_hex ("8520f0098930a754748b7ddcb43ef75a0dbf3a0d26381af4eba4a98eaa9b4e6a") ; let private_key = PrivateKey (decode_hex ("77076d0a7318a57d3c16c17251b26645df4c2f87ebc0992ab177fba51db92c2a" ,)) ; assert_eq ! (public_key_bytes , private_key . to_public ()) ; } }
};
}
