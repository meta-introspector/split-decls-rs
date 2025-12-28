macro_rules! padding_len {
    () => {
        # [doc = " Returns the number of additional bytes required to add to the given length"] # [doc = " in order to make the total length a multiple of 4. The return value is"] # [doc = " always less than 4."] pub (crate) fn padding_len (non_padding_len : usize) -> usize { (4 - (non_padding_len & 0b11)) & 0b11 }
    };
}

padding_len!()