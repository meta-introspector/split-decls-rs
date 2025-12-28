macro_rules! roundtrip_random_config {
    () => {
        fn roundtrip_random_config (input_len_range : Uniform < usize > , iterations : u32) { let mut input_buf : Vec < u8 > = Vec :: new () ; let mut encoded_buf = String :: new () ; let mut rng = rand :: rngs :: SmallRng :: from_entropy () ; for _ in 0 .. iterations { input_buf . clear () ; encoded_buf . clear () ; let input_len = input_len_range . sample (& mut rng) ; let engine = random_engine (& mut rng) ; for _ in 0 .. input_len { input_buf . push (rng . gen ()) ; } engine . encode_string (& input_buf , & mut encoded_buf) ; assert_encode_sanity (& encoded_buf , engine . config () . encode_padding () , input_len) ; assert_eq ! (input_buf , engine . decode (& encoded_buf) . unwrap ()) ; } }
    };
}

roundtrip_random_config!();