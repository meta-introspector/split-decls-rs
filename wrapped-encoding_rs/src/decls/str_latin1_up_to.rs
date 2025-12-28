macro_rules! str_latin1_up_to {
    () => {
        # [doc = " Returns the index of first byte that starts a non-Latin1 byte"] # [doc = " sequence, or the length of the string if there are none."] pub fn str_latin1_up_to (buffer : & str) -> usize { is_str_latin1_impl (buffer) . unwrap_or_else (| | buffer . len ()) }
    };
}

str_latin1_up_to!();