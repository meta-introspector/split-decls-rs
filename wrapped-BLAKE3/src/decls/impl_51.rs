macro_rules! deps {
    () => {
        Output!();
        Hash!();
        CVBytes!();
    };
}

macro_rules! impl_51 {
    () => {
        deps!();
        impl Output { fn chaining_value (& self) -> CVBytes { let mut cv = self . input_chaining_value ; self . platform . compress_in_place (& mut cv , & self . block , self . block_len , self . counter , self . flags ,) ; platform :: le_bytes_from_words_32 (& cv) } fn root_hash (& self) -> Hash { debug_assert_eq ! (self . counter , 0) ; let mut cv = self . input_chaining_value ; self . platform . compress_in_place (& mut cv , & self . block , self . block_len , 0 , self . flags | ROOT) ; Hash (platform :: le_bytes_from_words_32 (& cv)) } fn root_output_block (& self) -> [u8 ; 2 * OUT_LEN] { self . platform . compress_xof (& self . input_chaining_value , & self . block , self . block_len , self . counter , self . flags | ROOT ,) } }
    };
}

impl_51!()