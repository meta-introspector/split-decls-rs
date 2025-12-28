macro_rules! deps {
    () => {
        Unicode!();
    };
}

macro_rules! encode_latin1_lossy {
    () => {
        deps!();
        # [doc = " If the input is valid UTF-8 representing only Unicode code points from"] # [doc = " U+0000 to U+00FF, inclusive, converts the input into output that"] # [doc = " represents the value of each code point as the unsigned byte value of"] # [doc = " each output byte."] # [doc = ""] # [doc = " If the input does not fulfill the condition stated above, this function"] # [doc = " panics if debug assertions are enabled (and fuzzing isn't) and otherwise"] # [doc = " does something that is memory-safe without any promises about any"] # [doc = " properties of the output. In particular, callers shouldn't assume the"] # [doc = " output to be the same across crate versions or CPU architectures and"] # [doc = " should not assume that non-ASCII input can't map to ASCII output."] # [doc = ""] # [doc = " Borrows if input is ASCII-only. Performs a single heap allocation"] # [doc = " otherwise."] # [doc = ""] # [doc = " Only available if the `alloc` feature is enabled (enabled by default)."] # [cfg (feature = "alloc")] pub fn encode_latin1_lossy < 'a > (string : & 'a str) -> Cow < 'a , [u8] > { let bytes = string . as_bytes () ; let up_to = ascii_valid_up_to (bytes) ; if up_to >= bytes . len () { debug_assert_eq ! (up_to , bytes . len ()) ; return Cow :: Borrowed (bytes) ; } let (head , tail) = bytes . split_at (up_to) ; let capacity = bytes . len () ; let mut vec = Vec :: with_capacity (capacity) ; unsafe { vec . set_len (capacity) ; } (& mut vec [.. up_to]) . copy_from_slice (head) ; let written = convert_utf8_to_latin1_lossy (tail , & mut vec [up_to ..]) ; vec . truncate (up_to + written) ; Cow :: Owned (vec) }
    };
}

encode_latin1_lossy!();