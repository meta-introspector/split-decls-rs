macro_rules! deps {
    () => {
        Engine!();
    };
}

macro_rules! generate_random_encoded_data {
    () => {
        deps!();
        # [doc = " Returns a tuple of the original data length, the encoded data length (just data), and the length including padding."] # [doc = ""] # [doc = " Vecs provided should be empty."] fn generate_random_encoded_data < E : Engine , R : rand :: Rng , D : distributions :: Distribution < usize > > (engine : & E , orig_data : & mut Vec < u8 > , encode_buf : & mut Vec < u8 > , rng : & mut R , length_distribution : & D ,) -> (usize , usize , usize) { let padding : bool = engine . config () . encode_padding () ; let orig_len = fill_rand (orig_data , rng , length_distribution) ; let expected_encoded_len = encoded_len (orig_len , padding) . unwrap () ; encode_buf . resize (expected_encoded_len , 0) ; let base_encoded_len = engine . internal_encode (& orig_data [..] , & mut encode_buf [..]) ; let enc_len_with_padding = if padding { base_encoded_len + add_padding (base_encoded_len , & mut encode_buf [base_encoded_len ..]) } else { base_encoded_len } ; assert_eq ! (expected_encoded_len , enc_len_with_padding) ; (orig_len , base_encoded_len , enc_len_with_padding) }
    };
}

generate_random_encoded_data!();