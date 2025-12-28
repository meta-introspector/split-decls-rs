macro_rules! utf8_latin1_up_to {
    () => {
        # [doc = " Returns the index of first byte that starts an invalid byte"] # [doc = " sequence or a non-Latin1 byte sequence, or the length of the"] # [doc = " string if there are neither."] pub fn utf8_latin1_up_to (buffer : & [u8]) -> usize { is_utf8_latin1_impl (buffer) . unwrap_or (buffer . len ()) }
    };
}

utf8_latin1_up_to!()