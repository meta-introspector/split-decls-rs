macro_rules! deps {
    () => {
        Utf8Error!();
    };
}

macro_rules! impl_236 {
    () => {
        deps!();
        impl Utf8Error { # [doc = " Returns the byte index of the position immediately following the last"] # [doc = " valid UTF-8 byte."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " This examples shows how `valid_up_to` can be used to retrieve a"] # [doc = " possibly empty prefix that is guaranteed to be valid UTF-8:"] # [doc = ""] # [doc = " ```"] # [doc = " use bstr::ByteSlice;"] # [doc = ""] # [doc = " let s = b\"foobar\\xF1\\x80\\x80quux\";"] # [doc = " let err = s.to_str().unwrap_err();"] # [doc = ""] # [doc = " // This is guaranteed to never panic."] # [doc = " let string = s[..err.valid_up_to()].to_str().unwrap();"] # [doc = " assert_eq!(string, \"foobar\");"] # [doc = " ```"] # [inline] pub fn valid_up_to (& self) -> usize { self . valid_up_to } # [doc = " Returns the total number of invalid UTF-8 bytes immediately following"] # [doc = " the position returned by `valid_up_to`. This value is always at least"] # [doc = " `1`, but can be up to `3` if bytes form a valid prefix of some UTF-8"] # [doc = " encoded codepoint."] # [doc = ""] # [doc = " If the end of the original input was found before a valid UTF-8 encoded"] # [doc = " codepoint could be completed, then this returns `None`. This is useful"] # [doc = " when processing streams, where a `None` value signals that more input"] # [doc = " might be needed."] # [inline] pub fn error_len (& self) -> Option < usize > { self . error_len } }
    };
}

impl_236!();