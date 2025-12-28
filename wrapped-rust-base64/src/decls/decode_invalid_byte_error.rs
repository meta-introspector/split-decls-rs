macro_rules! deps {
    () => {
        DecodeError!();
        EngineWrapper!();
    };
}

macro_rules! decode_invalid_byte_error {
    () => {
        deps!();
        # [apply (all_engines)] fn decode_invalid_byte_error < E : EngineWrapper > (engine_wrapper : E) { let mut rng = seeded_rng () ; let mut orig_data = Vec :: < u8 > :: new () ; let mut encode_buf = Vec :: < u8 > :: new () ; let mut decode_buf = Vec :: < u8 > :: new () ; let len_range = distributions :: Uniform :: new (1 , 1_000) ; for _ in 0 .. 100_000 { let alphabet = random_alphabet (& mut rng) ; let engine = E :: random_alphabet (& mut rng , alphabet) ; orig_data . clear () ; encode_buf . clear () ; decode_buf . clear () ; let (orig_len , encoded_len_just_data , encoded_len_with_padding) = generate_random_encoded_data (& engine , & mut orig_data , & mut encode_buf , & mut rng , & len_range ,) ; decode_buf . resize (orig_len , 0) ; let invalid_byte : u8 = loop { let byte : u8 = rng . gen () ; if alphabet . symbols . contains (& byte) || byte == PAD_BYTE { continue ; } else { break byte ; } } ; let invalid_range = distributions :: Uniform :: new (0 , orig_len) ; let invalid_index = invalid_range . sample (& mut rng) ; encode_buf [invalid_index] = invalid_byte ; assert_eq ! (Err (DecodeError :: InvalidByte (invalid_index , invalid_byte)) , engine . decode_slice_unchecked (& encode_buf [0 .. encoded_len_with_padding] , & mut decode_buf [..] ,)) ; } }
    };
}

decode_invalid_byte_error!()