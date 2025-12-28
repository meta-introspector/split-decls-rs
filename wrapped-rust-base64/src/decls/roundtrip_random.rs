macro_rules! deps {
    () => {
        EngineWrapper!();
    };
}

macro_rules! roundtrip_random {
    () => {
        deps!();
        # [apply (all_engines)] fn roundtrip_random < E : EngineWrapper > (engine_wrapper : E) { let mut rng = seeded_rng () ; let mut orig_data = Vec :: < u8 > :: new () ; let mut encode_buf = Vec :: < u8 > :: new () ; let mut decode_buf = Vec :: < u8 > :: new () ; let len_range = distributions :: Uniform :: new (1 , 1_000) ; for _ in 0 .. 10_000 { let engine = E :: random (& mut rng) ; orig_data . clear () ; encode_buf . clear () ; decode_buf . clear () ; let (orig_len , _ , encoded_len) = generate_random_encoded_data (& engine , & mut orig_data , & mut encode_buf , & mut rng , & len_range ,) ; decode_buf . resize (orig_len , 0) ; let dec_len = engine . decode_slice_unchecked (& encode_buf [0 .. encoded_len] , & mut decode_buf [..]) . unwrap () ; assert_eq ! (orig_len , dec_len) ; assert_eq ! (& orig_data [..] , & decode_buf [.. dec_len]) ; } }
    };
}

roundtrip_random!();