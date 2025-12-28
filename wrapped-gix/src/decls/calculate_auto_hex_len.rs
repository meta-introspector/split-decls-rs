macro_rules! calculate_auto_hex_len {
    () => {
        fn calculate_auto_hex_len (num_packed_objects : u64) -> usize { let mut len = 64 - num_packed_objects . leading_zeros () ; len = len . div_ceil (2) ; len . max (7) as usize }
    };
}

calculate_auto_hex_len!()