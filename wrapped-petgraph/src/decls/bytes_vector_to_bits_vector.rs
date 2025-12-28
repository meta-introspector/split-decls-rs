macro_rules! bytes_vector_to_bits_vector {
    () => {
        fn bytes_vector_to_bits_vector (bytes : Vec < usize >) -> Vec < u8 > { bytes . iter () . flat_map (| & byte | get_number_as_bits (byte , 6)) . collect () }
    };
}

bytes_vector_to_bits_vector!();