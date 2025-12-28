macro_rules! deps {
    () => {
        HasherExt!();
        Hasher!();
        ChainingValue!();
    };
}

macro_rules! impl_45 {
    () => {
        deps!();
        impl HasherExt for Hasher { fn new_from_context_key (context_key : & [u8 ; KEY_LEN]) -> Hasher { let context_key_words = crate :: platform :: words_from_le_bytes_32 (context_key) ; Hasher :: new_internal (& context_key_words , crate :: DERIVE_KEY_MATERIAL) } fn set_input_offset (& mut self , offset : u64) -> & mut Hasher { assert_eq ! (self . count () , 0 , "hasher has already accepted input") ; assert_eq ! (offset % CHUNK_LEN as u64 , 0 , "offset ({offset}) must be a chunk boundary (divisible by {CHUNK_LEN})" ,) ; let counter = offset / CHUNK_LEN as u64 ; self . chunk_state . chunk_counter = counter ; self . initial_chunk_counter = counter ; self } fn finalize_non_root (& self) -> ChainingValue { assert_ne ! (self . count () , 0 , "empty subtrees are never valid") ; self . final_output () . chaining_value () } }
    };
}

impl_45!();