macro_rules! deps {
    () => {
        SecretKey!();
        PublicKey!();
    };
}

macro_rules! test_pem {
    () => {
        deps!();
        # [test] fn test_pem () { let sk_pem = "-----BEGIN PRIVATE KEY-----
MC4CAQAwBQYDK2VwBCIEIMXY1NUbUe/3dW2YUoKW5evsnCJPMfj60/q0RzGne3gg
-----END PRIVATE KEY-----\n" ; let sk = SecretKey :: from_pem (sk_pem) . unwrap () ; let pk_pem = "-----BEGIN PUBLIC KEY-----
MCowBQYDK2VwAyEAyrRjJfTnhMcW5igzYvPirFW5eUgMdKeClGzQhd4qw+Y=
-----END PUBLIC KEY-----\n" ; let pk = PublicKey :: from_pem (pk_pem) . unwrap () ; assert_eq ! (sk . public_key () , pk) ; # [cfg (feature = "std")] { let sk_pem2 = sk . to_pem () ; let pk_pem2 = pk . to_pem () ; assert_eq ! (sk_pem , sk_pem2) ; assert_eq ! (pk_pem , pk_pem2) ; } }
    };
}

test_pem!()