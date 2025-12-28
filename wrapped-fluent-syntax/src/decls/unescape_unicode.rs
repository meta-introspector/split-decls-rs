macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! unescape_unicode {
    () => {
        deps!();
        # [doc = " Unescapes to a writer without allocating."] # [doc = ""] # [doc = " ## Example"] # [doc = ""] # [doc = " ```"] # [doc = " use fluent_syntax::unicode::unescape_unicode;"] # [doc = ""] # [doc = " let mut s = String::new();"] # [doc = " unescape_unicode(&mut s, \"Foo \\\\U01F60A Bar\");"] # [doc = " assert_eq!(s, \"Foo 😊 Bar\");"] # [doc = " ```"] pub fn unescape_unicode < W > (w : & mut W , input : & str) -> fmt :: Result where W : fmt :: Write , { if unescape (w , input) ? { return Ok (()) ; } w . write_str (input) }
    };
}

unescape_unicode!();