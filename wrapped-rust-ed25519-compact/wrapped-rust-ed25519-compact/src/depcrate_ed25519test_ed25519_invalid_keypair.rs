// Generated macro for test_ed25519_invalid_keypair (function)
macro_rules! Depcrate_ed25519test_ed25519_invalid_keypair {
() => {
// Module: crate::ed25519
// Provides: {"test_ed25519_invalid_keypair"}
// Dependencies: {}
# [test] # [cfg (feature = "random")] fn test_ed25519_invalid_keypair () { let kp1 = KeyPair :: generate () ; let kp2 = KeyPair :: generate () ; assert_eq ! (kp1 . sk . validate_public_key (& kp2 . pk) . unwrap_err () , Error :: InvalidPublicKey) ; assert_eq ! (kp2 . sk . validate_public_key (& kp1 . pk) . unwrap_err () , Error :: InvalidPublicKey) ; assert ! (kp1 . sk . validate_public_key (& kp1 . pk) . is_ok ()) ; assert ! (kp2 . sk . validate_public_key (& kp2 . pk) . is_ok ()) ; assert ! (kp1 . validate () . is_ok ()) ; }
};
}
