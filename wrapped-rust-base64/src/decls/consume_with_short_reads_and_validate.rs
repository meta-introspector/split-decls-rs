macro_rules! consume_with_short_reads_and_validate {
    () => {
        fn consume_with_short_reads_and_validate < R : io :: Read > (rng : & mut rand :: rngs :: ThreadRng , expected_bytes : & [u8] , decoded : & mut [u8] , short_reader : & mut R ,) { let mut total_read = 0_usize ; loop { assert ! (total_read <= expected_bytes . len () , "tr {} size {}" , total_read , expected_bytes . len ()) ; if total_read == expected_bytes . len () { assert_eq ! (expected_bytes , & decoded [.. total_read]) ; assert_eq ! (0 , short_reader . read (& mut * decoded) . unwrap ()) ; assert_eq ! (expected_bytes , & decoded [.. total_read]) ; break ; } let decode_len = rng . gen_range (1 .. cmp :: max (2 , expected_bytes . len () * 2)) ; let read = short_reader . read (& mut decoded [total_read .. total_read + decode_len]) . unwrap () ; total_read += read ; } }
    };
}

consume_with_short_reads_and_validate!();