macro_rules! end_dec_digits {
    () => {
        # [doc = " Returns the index of the first non-underscore, non-decimal digit in `input`,"] # [doc = " or the `input.len()` if all characters are decimal digits."] pub (crate) fn end_dec_digits (input : & [u8]) -> usize { input . iter () . position (| b | ! matches ! (b , b'_' | b'0' ..= b'9')) . unwrap_or (input . len ()) }
    };
}

end_dec_digits!();