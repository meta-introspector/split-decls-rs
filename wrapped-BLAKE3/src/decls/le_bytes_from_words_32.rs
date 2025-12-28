macro_rules! le_bytes_from_words_32 {
    () => {
        # [inline (always)] pub fn le_bytes_from_words_32 (words : & [u32 ; 8]) -> [u8 ; 32] { let mut out = [0 ; 32] ; * array_mut_ref ! (out , 0 * 4 , 4) = words [0] . to_le_bytes () ; * array_mut_ref ! (out , 1 * 4 , 4) = words [1] . to_le_bytes () ; * array_mut_ref ! (out , 2 * 4 , 4) = words [2] . to_le_bytes () ; * array_mut_ref ! (out , 3 * 4 , 4) = words [3] . to_le_bytes () ; * array_mut_ref ! (out , 4 * 4 , 4) = words [4] . to_le_bytes () ; * array_mut_ref ! (out , 5 * 4 , 4) = words [5] . to_le_bytes () ; * array_mut_ref ! (out , 6 * 4 , 4) = words [6] . to_le_bytes () ; * array_mut_ref ! (out , 7 * 4 , 4) = words [7] . to_le_bytes () ; out }
    };
}

le_bytes_from_words_32!()