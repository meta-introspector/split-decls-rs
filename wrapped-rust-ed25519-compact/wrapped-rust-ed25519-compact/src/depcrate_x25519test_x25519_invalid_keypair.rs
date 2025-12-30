// Generated macro for test_x25519_invalid_keypair (function)
macro_rules! Depcrate_x25519test_x25519_invalid_keypair {
() => {
// Module: crate::x25519
// Provides: {"test_x25519_invalid_keypair"}
// Dependencies: {}
# [test] # [cfg (all (not (feature = "disable-signatures") , feature = "random"))] fn test_x25519_invalid_keypair () { let kp1 = KeyPair :: generate () ; let kp2 = KeyPair :: generate () ; assert_eq ! (kp1 . sk . validate_public_key (& kp2 . pk) . unwrap_err () , Error :: InvalidPublicKey) ; assert_eq ! (kp2 . sk . validate_public_key (& kp1 . pk) . unwrap_err () , Error :: InvalidPublicKey) ; assert ! (kp1 . sk . validate_public_key (& kp1 . pk) . is_ok ()) ; assert ! (kp2 . sk . validate_public_key (& kp2 . pk) . is_ok ()) ; assert ! (kp1 . validate () . is_ok ()) ; }
};
}
