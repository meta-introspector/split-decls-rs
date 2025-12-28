macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! decode_dict_size {
    () => {
        deps!();
        # [doc = " Decode LZIP dictionary size from encoded byte:"] # [doc = ""] # [doc = " The dictionary size is calculated by taking a power of 2 (the base size)"] # [doc = " and subtracting from it a fraction between 0/16 and 7/16 of the base size."] # [doc = ""] # [doc = " - Bits 4-0: contain the base 2 logarithm of the base size (12 to 29)"] # [doc = " - Bits 7-5: contain the numerator of the fraction (0 to 7) to subtract"] # [doc = ""] # [doc = " Example: 0xD3 = 2^19 - 6 * 2^15 = 512 KiB - 6 * 32 KiB = 320 KiB"] fn decode_dict_size (encoded : u8) -> Result < u32 > { let base_log2 = (encoded & 0x1F) as u32 ; let fraction_num = (encoded >> 5) as u32 ; if ! (12 ..= 29) . contains (& base_log2) { return Err (error_invalid_data ("invalid LZIP dictionary size base")) ; } if fraction_num > 7 { return Err (error_invalid_data ("invalid LZIP dictionary size fraction")) ; } let base_size = 1u32 << base_log2 ; let fraction_size = if base_log2 >= 4 { (base_size >> 4) * fraction_num } else { 0 } ; let dict_size = base_size - fraction_size ; if ! (MIN_DICT_SIZE ..= MAX_DICT_SIZE) . contains (& dict_size) { return Err (error_invalid_data ("LZIP dictionary size out of range")) ; } Ok (dict_size) }
    };
}

decode_dict_size!();