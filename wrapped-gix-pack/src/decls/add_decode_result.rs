macro_rules! deps {
    () => {
        Outcome!();
    };
}

macro_rules! add_decode_result {
    () => {
        deps!();
        fn add_decode_result (lhs : & mut data :: decode :: entry :: Outcome , rhs : data :: decode :: entry :: Outcome) { lhs . num_deltas += rhs . num_deltas ; lhs . decompressed_size += rhs . decompressed_size ; lhs . compressed_size += rhs . compressed_size ; lhs . object_size += rhs . object_size ; }
    };
}

add_decode_result!();