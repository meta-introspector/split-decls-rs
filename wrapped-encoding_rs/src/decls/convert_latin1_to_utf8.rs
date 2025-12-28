macro_rules! deps {
    () => {
        Unicode!();
    };
}

macro_rules! convert_latin1_to_utf8 {
    () => {
        deps!();
        # [doc = " Converts bytes whose unsigned value is interpreted as Unicode code point"] # [doc = " (i.e. U+0000 to U+00FF, inclusive) to UTF-8."] # [doc = ""] # [doc = " The length of the destination buffer must be at least the length of the"] # [doc = " source buffer times two."] # [doc = ""] # [doc = " Returns the number of bytes written."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if the destination buffer is shorter than stated above."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Note that this function may write garbage beyond the number of bytes"] # [doc = " indicated by the return value, so using a `&mut str` interpreted as"] # [doc = " `&mut [u8]` as the destination is not safe. If you want to convert into"] # [doc = " a `&mut str`, use `convert_utf16_to_str()` instead of this function."] # [inline] pub fn convert_latin1_to_utf8 (src : & [u8] , dst : & mut [u8]) -> usize { assert ! (dst . len () >= src . len () * 2 , "Destination must not be shorter than the source times two.") ; let (read , written) = convert_latin1_to_utf8_partial (src , dst) ; debug_assert_eq ! (read , src . len ()) ; written }
    };
}

convert_latin1_to_utf8!()