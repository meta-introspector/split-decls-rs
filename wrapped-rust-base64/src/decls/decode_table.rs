macro_rules! deps {
    () => {
        Alphabet!();
    };
}

macro_rules! decode_table {
    () => {
        deps!();
        # [doc = " Returns a table mapping base64 bytes as the lookup index to either:"] # [doc = " - [`INVALID_VALUE`] for bytes that aren't members of the alphabet"] # [doc = " - a byte whose lower 6 bits are the value that was encoded into the index byte"] pub (crate) const fn decode_table (alphabet : & Alphabet) -> [u8 ; 256] { let mut decode_table = [INVALID_VALUE ; 256] ; let mut index = 0 ; while index < 64 { decode_table [alphabet . symbols [index] as usize] = index as u8 ; index += 1 ; } decode_table }
    };
}

decode_table!()