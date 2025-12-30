// Generated macro for StrComparison (struct)
macro_rules! DepcrateStrComparison {
() => {
// Module: crate
// Provides: {"StrComparison"}
// Dependencies: {}
# [doc = " A comparison of two strings."] # [doc = ""] # [doc = " In contrast to [`Comparison`], which uses the [`core::fmt::Debug`] representation,"] # [doc = " `StrComparison` uses the string values directly, resulting in multi-line output for multiline strings."] # [doc = ""] # [doc = " ```"] # [doc = " use pretty_assertions::StrComparison;"] # [doc = ""] # [doc = " print!(\"{}\", StrComparison::new(\"foo\\nbar\", \"foo\\nbaz\"));"] # [doc = " ```"] # [doc = ""] # [doc = " ## Value type bounds"] # [doc = ""] # [doc = " Any value that can be referenced as a [`str`] via [`AsRef`] may be used:"] # [doc = ""] # [doc = " ```"] # [doc = " use pretty_assertions::StrComparison;"] # [doc = ""] # [doc = " #[derive(PartialEq)]"] # [doc = " struct MyString(String);"] # [doc = ""] # [doc = " impl AsRef<str> for MyString {"] # [doc = "     fn as_ref(&self) -> &str {"] # [doc = "         &self.0"] # [doc = "     }"] # [doc = " }"] # [doc = ""] # [doc = " print!("] # [doc = "     \"{}\","] # [doc = "     StrComparison::new("] # [doc = "         &MyString(\"foo\\nbar\".to_owned()),"] # [doc = "         &MyString(\"foo\\nbaz\".to_owned()),"] # [doc = "     ),"] # [doc = " );"] # [doc = " ```"] # [doc = ""] # [doc = " The values may have different types, although in practice they are usually the same."] pub struct StrComparison < 'a , TLeft , TRight > where TLeft : ? Sized , TRight : ? Sized , { left : & 'a TLeft , right : & 'a TRight , }
};
}
