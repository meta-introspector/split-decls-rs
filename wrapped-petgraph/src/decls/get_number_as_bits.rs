macro_rules! get_number_as_bits {
    () => {
        fn get_number_as_bits (n : usize , bits_length : usize) -> Vec < usize > { let mut bits = Vec :: new () ; for i in (0 .. bits_length) . rev () { bits . push ((n >> i) & 1) ; } bits }
    };
}

get_number_as_bits!();