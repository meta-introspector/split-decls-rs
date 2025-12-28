macro_rules! convert_utf8_to_utf16_without_replacement {
    () => {
        # [doc = " Converts potentially-invalid UTF-8 to valid UTF-16 signaling on error."] # [doc = ""] # [doc = " The length of the destination buffer must be at least the length of the"] # [doc = " source buffer."] # [doc = ""] # [doc = " Returns the number of `u16`s written or `None` if the input was invalid."] # [doc = ""] # [doc = " When the input was invalid, some output may have been written."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if the destination buffer is shorter than stated above."] pub fn convert_utf8_to_utf16_without_replacement (src : & [u8] , dst : & mut [u16]) -> Option < usize > { assert ! (dst . len () >= src . len () , "Destination must not be shorter than the source.") ; let (read , written) = convert_utf8_to_utf16_up_to_invalid (src , dst) ; if read == src . len () { return Some (written) ; } None }
    };
}

convert_utf8_to_utf16_without_replacement!()