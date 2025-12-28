macro_rules! deps {
    () => {
        EngineWrapper!();
    };
}

macro_rules! encode_doesnt_write_extra_bytes {
    () => {
        deps!();
        # [apply (all_engines)] fn encode_doesnt_write_extra_bytes < E : EngineWrapper > (engine_wrapper : E) { let mut rng = seeded_rng () ; let mut orig_data = Vec :: < u8 > :: new () ; let mut encode_buf = Vec :: < u8 > :: new () ; let mut encode_buf_backup = Vec :: < u8 > :: new () ; let input_len_range = distributions :: Uniform :: new (0 , 1000) ; for _ in 0 .. 10_000 { let engine = E :: random (& mut rng) ; let padded = engine . config () . encode_padding () ; orig_data . clear () ; encode_buf . clear () ; encode_buf_backup . clear () ; let orig_len = fill_rand (& mut orig_data , & mut rng , & input_len_range) ; let prefix_len = 1024 ; fill_rand_len (& mut encode_buf , & mut rng , prefix_len * 2 + orig_len * 2) ; encode_buf_backup . extend_from_slice (& encode_buf [..]) ; let expected_encode_len_no_pad = encoded_len (orig_len , false) . unwrap () ; let encoded_len_no_pad = engine . internal_encode (& orig_data [..] , & mut encode_buf [prefix_len ..]) ; assert_eq ! (expected_encode_len_no_pad , encoded_len_no_pad) ; assert_eq ! (& encode_buf_backup [.. prefix_len] , & encode_buf [.. prefix_len]) ; assert_eq ! (& encode_buf_backup [(prefix_len + encoded_len_no_pad) ..] , & encode_buf [(prefix_len + encoded_len_no_pad) ..]) ; let encoded_data = & encode_buf [prefix_len .. (prefix_len + encoded_len_no_pad)] ; assert_encode_sanity (std :: str :: from_utf8 (encoded_data) . unwrap () , false , orig_len ,) ; let pad_len = if padded { add_padding (encoded_len_no_pad , & mut encode_buf [prefix_len + encoded_len_no_pad ..] ,) } else { 0 } ; assert_eq ! (orig_data , engine . decode (& encode_buf [prefix_len .. (prefix_len + encoded_len_no_pad + pad_len)] ,) . unwrap ()) ; } }
    };
}

encode_doesnt_write_extra_bytes!()