macro_rules! utf16_valid_up_to {
    () => {
        # [doc = " Returns the index of the first unpaired surrogate or, if the input is"] # [doc = " valid UTF-16 in its entirety, the length of the input."] pub fn utf16_valid_up_to (buffer : & [u16]) -> usize { utf16_valid_up_to_impl (buffer) }
    };
}

utf16_valid_up_to!()