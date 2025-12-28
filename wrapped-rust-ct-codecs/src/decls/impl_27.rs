macro_rules! deps {
    () => {
        Hex!();
        Decoder!();
        Error!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        impl Decoder for Hex { # [doc = " Decodes hexadecimal data back into its binary representation."] # [doc = ""] # [doc = " The decoding is performed in constant time relative to the input length."] # [doc = " Both uppercase and lowercase hexadecimal characters are accepted."] # [doc = ""] # [doc = " # Arguments"] # [doc = ""] # [doc = " * `bin` - Mutable buffer to store the decoded output"] # [doc = " * `hex` - Hexadecimal input data to decode"] # [doc = " * `ignore` - Optional set of characters to ignore during decoding"] # [doc = ""] # [doc = " # Returns"] # [doc = ""] # [doc = " * `Ok(&[u8])` - A slice of the binary buffer containing the decoded data"] # [doc = " * `Err(Error::Overflow)` - If the output buffer is too small"] # [doc = " * `Err(Error::InvalidInput)` - If the input contains invalid characters or has odd length"] fn decode < 't , IN : AsRef < [u8] > > (bin : & 't mut [u8] , hex : IN , ignore : Option < & [u8] > ,) -> Result < & 't [u8] , Error > { let hex = hex . as_ref () ; let bin_maxlen = bin . len () ; let mut bin_pos = 0 ; let mut state = false ; let mut c_acc = 0 ; for & c in hex { let c_num = c ^ 48 ; let c_num0 = ((c_num as u16) . wrapping_sub (10) >> 8) as u8 ; let c_alpha = (c & ! 32) . wrapping_sub (55) ; let c_alpha0 = (((c_alpha as u16) . wrapping_sub (10) ^ ((c_alpha as u16) . wrapping_sub (16))) >> 8) as u8 ; if (c_num0 | c_alpha0) == 0 { match ignore { Some (ignore) if ignore . contains (& c) => continue , _ => return Err (Error :: InvalidInput) , } ; } let c_val = (c_num0 & c_num) | (c_alpha0 & c_alpha) ; if bin_pos >= bin_maxlen { return Err (Error :: Overflow) ; } if ! state { c_acc = c_val << 4 ; } else { bin [bin_pos] = c_acc | c_val ; bin_pos += 1 ; } state = ! state ; } if state { return Err (Error :: InvalidInput) ; } Ok (& bin [.. bin_pos]) } }
    };
}

impl_27!()