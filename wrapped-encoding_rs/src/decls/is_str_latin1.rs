macro_rules! is_str_latin1 {
    () => {
        # [doc = " Checks whether the buffer represents only code points less than or equal"] # [doc = " to U+00FF."] # [doc = ""] # [doc = " Fails fast. (I.e. returns before having read the whole buffer if code"] # [doc = " points above U+00FF are discovered."] pub fn is_str_latin1 (buffer : & str) -> bool { is_str_latin1_impl (buffer) . is_none () }
    };
}

is_str_latin1!();