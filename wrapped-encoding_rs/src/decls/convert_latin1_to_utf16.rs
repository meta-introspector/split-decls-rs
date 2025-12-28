macro_rules! deps {
    () => {
        Unicode!();
    };
}

macro_rules! convert_latin1_to_utf16 {
    () => {
        deps!();
        # [doc = " Converts bytes whose unsigned value is interpreted as Unicode code point"] # [doc = " (i.e. U+0000 to U+00FF, inclusive) to UTF-16."] # [doc = ""] # [doc = " The length of the destination buffer must be at least the length of the"] # [doc = " source buffer."] # [doc = ""] # [doc = " The number of `u16`s written equals the length of the source buffer."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if the destination buffer is shorter than stated above."] pub fn convert_latin1_to_utf16 (src : & [u8] , dst : & mut [u16]) { assert ! (dst . len () >= src . len () , "Destination must not be shorter than the source.") ; unsafe { unpack_latin1 (src . as_ptr () , dst . as_mut_ptr () , src . len ()) ; } }
    };
}

convert_latin1_to_utf16!();