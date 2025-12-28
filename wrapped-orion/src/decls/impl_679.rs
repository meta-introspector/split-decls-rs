macro_rules! deps {
    () => {
        KemTester!();
        TestableKem!();
    };
}

macro_rules! impl_679 {
    () => {
        deps!();
        impl < T , K , C > KemTester < T , K , C > where T : TestableKem < K , C > , K : PartialEq + core :: fmt :: Debug , C : PartialEq + core :: fmt :: Debug + AsRef < [u8] > , { pub fn run_all_tests (seed : & [u8]) { Self :: keygen_encap_decap_roundtrip (seed) ; Self :: decap_wrong_key_implicit_reject (seed) ; Self :: encap_twice_then_decap (seed) ; Self :: decap_wrong_ciphertext_implicit_reject (seed) ; } fn keygen_encap_decap_roundtrip (seed : & [u8]) { let (ek1 , dk1) = T :: keygen (seed) . unwrap () ; for _ in 0 .. 100 { let (k , c) = T :: encap (& ek1) . unwrap () ; let k_prime = T :: decap (& dk1 , & c) . unwrap () ; assert_eq ! (k , k_prime) ; } } fn decap_wrong_key_implicit_reject (seed : & [u8]) { let (ek1 , _) = T :: keygen (seed) . unwrap () ; let mut seed_mod = seed . to_vec () ; seed_mod [0] ^= 1 ; let (_ , dk2) = T :: keygen (& seed_mod) . unwrap () ; let (k , c) = T :: encap (& ek1) . unwrap () ; let k_prime = T :: decap (& dk2 , & c) . unwrap () ; assert_ne ! (k , k_prime) ; } fn decap_wrong_ciphertext_implicit_reject (seed : & [u8]) { let (ek1 , dk1) = T :: keygen (seed) . unwrap () ; let (k , c) = T :: encap (& ek1) . unwrap () ; let mut c_mod = c . as_ref () . to_vec () ; c_mod [0] ^= 1 ; let k_prime = T :: decap (& dk1 , & T :: ciphertext_from_bytes (& c_mod) . unwrap ()) . unwrap () ; assert_ne ! (k , k_prime) ; } fn encap_twice_then_decap (seed : & [u8]) { let (ek1 , dk1) = T :: keygen (seed) . unwrap () ; let (k1 , c1) = T :: encap (& ek1) . unwrap () ; let (k2 , c2) = T :: encap (& ek1) . unwrap () ; assert_ne ! (k1 , k2) ; assert_ne ! (c1 , c2) ; let k1_prime = T :: decap (& dk1 , & c1) . unwrap () ; let k2_prime = T :: decap (& dk1 , & c2) . unwrap () ; assert_ne ! (k1_prime , k2_prime) ; assert_eq ! (k1_prime , T :: decap (& dk1 , & c1) . unwrap ()) ; assert_eq ! (k2_prime , T :: decap (& dk1 , & c2) . unwrap ()) ; } }
    };
}

impl_679!()