macro_rules! deps {
    () => {
        DecodeError!();
        DecodeSliceError!();
    };
}

macro_rules! complete_quads_len {
    () => {
        deps!();
        # [doc = " Returns the length of complete quads, except for the last one, even if it is complete."] # [doc = ""] # [doc = " Returns an error if the output len is not big enough for decoding those complete quads, or if"] # [doc = " the input % 4 == 1, and that last byte is an invalid value other than a pad byte."] # [doc = ""] # [doc = " - `input` is the base64 input"] # [doc = " - `input_len_rem` is input len % 4"] # [doc = " - `output_len` is the length of the output slice"] pub (crate) fn complete_quads_len (input : & [u8] , input_len_rem : usize , output_len : usize , decode_table : & [u8 ; 256] ,) -> Result < usize , DecodeSliceError > { debug_assert ! (input . len () % 4 == input_len_rem) ; if input_len_rem == 1 { let last_byte = input [input . len () - 1] ; if last_byte != PAD_BYTE && decode_table [usize :: from (last_byte)] == INVALID_VALUE { return Err (DecodeError :: InvalidByte (input . len () - 1 , last_byte) . into ()) ; } } ; let input_complete_nonterminal_quads_len = input . len () . saturating_sub (input_len_rem) . saturating_sub (usize :: from (input_len_rem == 0) * 4) ; debug_assert ! (input . is_empty () || (1 ..= 4) . contains (& (input . len () - input_complete_nonterminal_quads_len))) ; if output_len < input_complete_nonterminal_quads_len / 4 * 3 { return Err (DecodeSliceError :: OutputSliceTooSmall) ; } ; Ok (input_complete_nonterminal_quads_len) }
    };
}

complete_quads_len!();