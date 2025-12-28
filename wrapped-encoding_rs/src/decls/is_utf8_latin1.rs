macro_rules! is_utf8_latin1 {
    () => {
        # [doc = " Checks whether the buffer is valid UTF-8 representing only code points"] # [doc = " less than or equal to U+00FF."] # [doc = ""] # [doc = " Fails fast. (I.e. returns before having read the whole buffer if UTF-8"] # [doc = " invalidity or code points above U+00FF are discovered."] pub fn is_utf8_latin1 (buffer : & [u8]) -> bool { is_utf8_latin1_impl (buffer) . is_none () }
    };
}

is_utf8_latin1!();