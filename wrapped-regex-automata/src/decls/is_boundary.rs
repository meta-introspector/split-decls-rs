macro_rules! is_boundary {
    () => {
        # [doc = " Returns true if and only if the given offset in the given bytes falls on a"] # [doc = " valid UTF-8 encoded codepoint boundary."] # [doc = ""] # [doc = " If `bytes` is not valid UTF-8, then the behavior of this routine is"] # [doc = " unspecified."] # [cfg_attr (feature = "perf-inline" , inline (always))] pub (crate) fn is_boundary (bytes : & [u8] , i : usize) -> bool { match bytes . get (i) { None => i == bytes . len () , Some (& b) => b <= 0b0111_1111 || b >= 0b1100_0000 , } }
    };
}

is_boundary!()