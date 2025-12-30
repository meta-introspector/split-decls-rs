// Generated macro for unescape_unicode_to_string (function)
macro_rules! Depcrate_unicodeunescape_unicode_to_string {
() => {
// Module: crate::unicode
// Provides: {"unescape_unicode_to_string"}
// Dependencies: {}
# [doc = " Unescapes to a `Cow<str>` optionally allocating."] # [doc = ""] # [doc = " ## Example"] # [doc = ""] # [doc = " ```"] # [doc = " use fluent_syntax::unicode::unescape_unicode_to_string;"] # [doc = ""] # [doc = " assert_eq!("] # [doc = "     unescape_unicode_to_string(\"Foo \\\\U01F60A Bar\"),"] # [doc = "     \"Foo 😊 Bar\""] # [doc = " );"] # [doc = " ```"] pub fn unescape_unicode_to_string (input : & str) -> Cow < '_ , str > { let mut result = String :: new () ; let owned = unescape (& mut result , input) . expect ("String write methods don't Err") ; if owned { Cow :: Owned (result) } else { Cow :: Borrowed (input) } }
};
}
