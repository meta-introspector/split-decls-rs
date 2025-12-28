macro_rules! deps {
    () => {
        EngineWrapper!();
    };
}

macro_rules! decode_into_slice_fits_in_precisely_sized_slice {
    () => {
        deps!();
        # [apply (all_engines)] fn decode_into_slice_fits_in_precisely_sized_slice < E : EngineWrapper > (engine_wrapper : E) { let mut orig_data = Vec :: new () ; let mut encoded_data = String :: new () ; let mut decode_buf = Vec :: new () ; let input_len_range = distributions :: Uniform :: new (0 , 1000) ; let mut rng = rngs :: SmallRng :: from_entropy () ; for _ in 0 .. 10_000 { orig_data . clear () ; encoded_data . clear () ; decode_buf . clear () ; let input_len = input_len_range . sample (& mut rng) ; for _ in 0 .. input_len { orig_data . push (rng . gen ()) ; } let engine = E :: random (& mut rng) ; engine . encode_string (& orig_data , & mut encoded_data) ; assert_encode_sanity (& encoded_data , engine . config () . encode_padding () , input_len) ; decode_buf . resize (input_len , 0) ; let decode_bytes_written = engine . decode_slice_unchecked (encoded_data . as_bytes () , & mut decode_buf [..]) . unwrap () ; assert_eq ! (orig_data . len () , decode_bytes_written) ; assert_eq ! (orig_data , decode_buf) ; decode_buf . clear () ; decode_buf . resize (input_len , 0) ; let decode_bytes_written = engine . decode_slice (encoded_data . as_bytes () , & mut decode_buf [..]) . unwrap () ; assert_eq ! (orig_data . len () , decode_bytes_written) ; assert_eq ! (orig_data , decode_buf) ; } }
    };
}

decode_into_slice_fits_in_precisely_sized_slice!()