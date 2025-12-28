macro_rules! deps {
    () => {
        Alphabet!();
    };
}

macro_rules! encode_table {
    () => {
        deps!();
        # [doc = " Returns a table mapping a 6-bit index to the ASCII byte encoding of the index"] pub (crate) const fn encode_table (alphabet : & Alphabet) -> [u8 ; 64] { let mut encode_table = [0_u8 ; 64] ; { let mut index = 0 ; while index < 64 { encode_table [index] = alphabet . symbols [index] ; index += 1 ; } } encode_table }
    };
}

encode_table!()