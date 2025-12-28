macro_rules! deps {
    () => {
        KeyPair!();
    };
}

macro_rules! test_x25519_map {
    () => {
        deps!();
        # [cfg (not (feature = "disable-signatures"))] # [test] fn test_x25519_map () { use super :: KeyPair as EdKeyPair ; let edkp_a = EdKeyPair :: generate () ; let edkp_b = EdKeyPair :: generate () ; let kp_a = KeyPair :: from_ed25519 (& edkp_a) . unwrap () ; let kp_b = KeyPair :: from_ed25519 (& edkp_b) . unwrap () ; let output_a = kp_b . pk . dh (& kp_a . sk) . unwrap () ; let output_b = kp_a . pk . dh (& kp_b . sk) . unwrap () ; assert_eq ! (output_a , output_b) ; }
    };
}

test_x25519_map!();