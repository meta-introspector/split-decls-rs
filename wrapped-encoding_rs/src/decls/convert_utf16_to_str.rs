macro_rules! convert_utf16_to_str {
    () => {
        # [doc = " Converts potentially-invalid UTF-16 to valid UTF-8 with errors replaced"] # [doc = " with the REPLACEMENT CHARACTER such that the validity of the output is"] # [doc = " signaled using the Rust type system."] # [doc = ""] # [doc = " The length of the destination buffer must be at least the length of the"] # [doc = " source buffer times three."] # [doc = ""] # [doc = " Returns the number of bytes written."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if the destination buffer is shorter than stated above."] # [inline (always)] pub fn convert_utf16_to_str (src : & [u16] , dst : & mut str) -> usize { assert ! (dst . len () >= src . len () * 3) ; let (read , written) = convert_utf16_to_str_partial (src , dst) ; debug_assert_eq ! (read , src . len ()) ; written }
    };
}

convert_utf16_to_str!();