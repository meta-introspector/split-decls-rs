macro_rules! unescape_unicode_to_string {
    () => {
        # [doc = " Unescapes to a `Cow<str>` optionally allocating."] # [doc = ""] # [doc = " ## Example"] # [doc = ""] # [doc = " ```"] # [doc = " use fluent_syntax::unicode::unescape_unicode_to_string;"] # [doc = ""] # [doc = " assert_eq!("] # [doc = "     unescape_unicode_to_string(\"Foo \\\\U01F60A Bar\"),"] # [doc = "     \"Foo 😊 Bar\""] # [doc = " );"] # [doc = " ```"] pub fn unescape_unicode_to_string (input : & str) -> Cow < '_ , str > { let mut result = String :: new () ; let owned = unescape (& mut result , input) . expect ("String write methods don't Err") ; if owned { Cow :: Owned (result) } else { Cow :: Borrowed (input) } }
    };
}

unescape_unicode_to_string!()