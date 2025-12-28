macro_rules! deps {
    () => {
        Outcome!();
    };
}

macro_rules! div_decode_result {
    () => {
        deps!();
        fn div_decode_result (lhs : & mut data :: decode :: entry :: Outcome , div : usize) { if div != 0 { lhs . num_deltas = (lhs . num_deltas as f32 / div as f32) as u32 ; lhs . decompressed_size /= div as u64 ; lhs . compressed_size /= div ; lhs . object_size /= div as u64 ; } }
    };
}

div_decode_result!();