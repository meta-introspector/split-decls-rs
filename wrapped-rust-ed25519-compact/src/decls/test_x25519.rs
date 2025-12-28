macro_rules! deps {
    () => {
        SecretKey!();
        KeyPair!();
        PublicKey!();
    };
}

macro_rules! test_x25519 {
    () => {
        deps!();
        # [test] fn test_x25519 () { let sk_1 = SecretKey :: from_slice (& [1u8 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ,]) . unwrap () ; let output = PublicKey :: base_point () . unclamped_mul (& sk_1) . unwrap () ; assert_eq ! (PublicKey :: from (output) , PublicKey :: base_point ()) ; let kp_a = KeyPair :: generate () ; let kp_b = KeyPair :: generate () ; let output_a = kp_b . pk . dh (& kp_a . sk) . unwrap () ; let output_b = kp_a . pk . dh (& kp_b . sk) . unwrap () ; assert_eq ! (output_a , output_b) ; }
    };
}

test_x25519!();