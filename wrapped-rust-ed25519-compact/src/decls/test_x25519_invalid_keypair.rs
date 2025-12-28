macro_rules! deps {
    () => {
        Error!();
        KeyPair!();
    };
}

macro_rules! test_x25519_invalid_keypair {
    () => {
        deps!();
        # [test] # [cfg (all (not (feature = "disable-signatures") , feature = "random"))] fn test_x25519_invalid_keypair () { let kp1 = KeyPair :: generate () ; let kp2 = KeyPair :: generate () ; assert_eq ! (kp1 . sk . validate_public_key (& kp2 . pk) . unwrap_err () , Error :: InvalidPublicKey) ; assert_eq ! (kp2 . sk . validate_public_key (& kp1 . pk) . unwrap_err () , Error :: InvalidPublicKey) ; assert ! (kp1 . sk . validate_public_key (& kp1 . pk) . is_ok ()) ; assert ! (kp2 . sk . validate_public_key (& kp2 . pk) . is_ok ()) ; assert ! (kp1 . validate () . is_ok ()) ; }
    };
}

test_x25519_invalid_keypair!()