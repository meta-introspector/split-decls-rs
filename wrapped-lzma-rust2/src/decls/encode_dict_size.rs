macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! encode_dict_size {
    () => {
        deps!();
        # [doc = " Encode dictionary size to LZIP format."] # [doc = ""] # [doc = " The dictionary size is encoded as:"] # [doc = " - Bits 4-0: base 2 logarithm of the base size (12 to 29)"] # [doc = " - Bits 7-5: numerator of the fraction (0 to 7) to subtract from base size"] fn encode_dict_size (dict_size : u32) -> Result < u8 > { if ! (MIN_DICT_SIZE ..= MAX_DICT_SIZE) . contains (& dict_size) { return Err (error_invalid_input ("LZIP dictionary size out of valid range" ,)) ; } let mut base_log2 = 32 - dict_size . leading_zeros () - 1 ; if (1u32 << base_log2) < dict_size { base_log2 += 1 ; } if base_log2 < 12 { base_log2 = 12 ; } if base_log2 > 29 { return Err (error_invalid_input ("dictionary size too large")) ; } let base_size = 1u32 << base_log2 ; let mut fraction_num = 0u32 ; if base_size > dict_size { let diff = base_size - dict_size ; let fraction_unit = base_size >> 4 ; if fraction_unit > 0 { fraction_num = diff . div_ceil (fraction_unit) ; if fraction_num > 7 { base_log2 += 1 ; if base_log2 > 29 { return Err (error_invalid_input ("dictionary size too large")) ; } fraction_num = 0 ; } } } Ok (((fraction_num << 5) | (base_log2 & 0x1F)) as u8) }
    };
}

encode_dict_size!()