macro_rules! define_substring_forward_quickcheck {
    () => {
        # [doc = " $fwd is a `impl FnMut(haystack, needle) -> Option<Option<usize>>`. When the"] # [doc = " routine returns `None`, then it's skipped, which is useful for substring"] # [doc = " implementations that don't work for all inputs."] # [macro_export] macro_rules ! define_substring_forward_quickcheck { ($ fwd : expr) => { # [cfg (not (miri))] quickcheck :: quickcheck ! { fn qc_fwd_prefix_is_substring (bs : alloc :: vec :: Vec < u8 >) -> bool { crate :: tests :: substring :: prop :: prefix_is_substring (& bs , $ fwd) } fn qc_fwd_suffix_is_substring (bs : alloc :: vec :: Vec < u8 >) -> bool { crate :: tests :: substring :: prop :: suffix_is_substring (& bs , $ fwd) } fn qc_fwd_matches_naive (haystack : alloc :: vec :: Vec < u8 >, needle : alloc :: vec :: Vec < u8 >) -> bool { crate :: tests :: substring :: prop :: same_as_naive (false , & haystack , & needle , $ fwd ,) } } } ; }
    };
}

define_substring_forward_quickcheck!()