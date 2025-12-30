// Generated macro for from_cesu8 (function)
macro_rules! Depcratefrom_cesu8 {
() => {
// Module: crate
// Provides: {"from_cesu8"}
// Dependencies: {}
# [doc = " Convert CESU-8 data to a Rust string, re-encoding only if necessary."] # [doc = " Returns an error if the data cannot be represented as valid UTF-8."] # [doc = ""] # [doc = " ```"] # [doc = " use std::borrow::Cow;"] # [doc = " use cesu8::from_cesu8;"] # [doc = ""] # [doc = " // This string is valid as UTF-8 or CESU-8, so it doesn't change,"] # [doc = " // and we can convert it without allocating memory."] # [doc = " assert_eq!(Cow::Borrowed(\"aé日\"),"] # [doc = "            from_cesu8(\"aé日\".as_bytes()).unwrap());"] # [doc = ""] # [doc = " // This string is CESU-8 data containing a 6-byte surrogate pair,"] # [doc = " // which becomes a 4-byte UTF-8 string."] # [doc = " let data = &[0xED, 0xA0, 0x81, 0xED, 0xB0, 0x81];"] # [doc = " assert_eq!(Cow::Borrowed(\"\\u{10401}\"),"] # [doc = "            from_cesu8(data).unwrap());"] # [doc = " ```"] pub fn from_cesu8 (bytes : & [u8]) -> Result < Cow < str > , Cesu8DecodingError > { from_cesu8_internal (bytes , Variant :: Standard) }
};
}
