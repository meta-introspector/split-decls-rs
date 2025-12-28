macro_rules! deps {
    () => {
        PublicKey!();
        KeyPair!();
        SecretKey!();
    };
}

macro_rules! test_der {
    () => {
        deps!();
        # [test] fn test_der () { let kp = KeyPair :: generate () ; let sk_der = kp . sk . to_der () ; let sk2 = SecretKey :: from_der (& sk_der) . unwrap () ; let pk_der = kp . pk . to_der () ; let pk2 = PublicKey :: from_der (& pk_der) . unwrap () ; assert_eq ! (kp . sk , sk2) ; assert_eq ! (kp . pk , pk2) ; }
    };
}

test_der!();