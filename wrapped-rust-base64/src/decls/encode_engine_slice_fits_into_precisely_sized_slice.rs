macro_rules! deps {
    () => {
        EngineWrapper!();
    };
}

macro_rules! encode_engine_slice_fits_into_precisely_sized_slice {
    () => {
        deps!();
        # [apply (all_engines)] fn encode_engine_slice_fits_into_precisely_sized_slice < E : EngineWrapper > (engine_wrapper : E) { let mut orig_data = Vec :: new () ; let mut encoded_data = Vec :: new () ; let mut decoded = Vec :: new () ; let input_len_range = distributions :: Uniform :: new (0 , 1000) ; let mut rng = rngs :: SmallRng :: from_entropy () ; for _ in 0 .. 10_000 { orig_data . clear () ; encoded_data . clear () ; decoded . clear () ; let input_len = input_len_range . sample (& mut rng) ; for _ in 0 .. input_len { orig_data . push (rng . gen ()) ; } let engine = E :: random (& mut rng) ; let encoded_size = encoded_len (input_len , engine . config () . encode_padding ()) . unwrap () ; encoded_data . resize (encoded_size , 0) ; assert_eq ! (encoded_size , engine . encode_slice (& orig_data , & mut encoded_data) . unwrap ()) ; assert_encode_sanity (std :: str :: from_utf8 (& encoded_data [0 .. encoded_size]) . unwrap () , engine . config () . encode_padding () , input_len ,) ; engine . decode_vec (& encoded_data [0 .. encoded_size] , & mut decoded) . unwrap () ; assert_eq ! (orig_data , decoded) ; } }
    };
}

encode_engine_slice_fits_into_precisely_sized_slice!();