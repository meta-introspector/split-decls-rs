// Generated macro for ByteString (struct)
macro_rules! Depcrate_bstrByteString {
() => {
// Module: crate::bstr
// Provides: {"ByteString"}
// Dependencies: {}
# [doc = " A wrapper for `Vec<u8>` representing a human-readable string that's conventionally, but not"] # [doc = " always, UTF-8."] # [doc = ""] # [doc = " Unlike `String`, this type permits non-UTF-8 contents, making it suitable for user input,"] # [doc = " non-native filenames (as `Path` only supports native filenames), and other applications that"] # [doc = " need to round-trip whatever data the user provides."] # [doc = ""] # [doc = " A `ByteString` owns its contents and can grow and shrink, like a `Vec` or `String`. For a"] # [doc = " borrowed byte string, see [`ByteStr`](../../std/bstr/struct.ByteStr.html)."] # [doc = ""] # [doc = " `ByteString` implements `Deref` to `&Vec<u8>`, so all methods available on `&Vec<u8>` are"] # [doc = " available on `ByteString`. Similarly, `ByteString` implements `DerefMut` to `&mut Vec<u8>`,"] # [doc = " so you can modify a `ByteString` using any method available on `&mut Vec<u8>`."] # [doc = ""] # [doc = " The `Debug` and `Display` implementations for `ByteString` are the same as those for `ByteStr`,"] # [doc = " showing invalid UTF-8 as hex escapes or the Unicode replacement character, respectively."] # [unstable (feature = "bstr" , issue = "134915")] # [repr (transparent)] # [derive (Clone)] # [doc (alias = "BString")] pub struct ByteString (pub Vec < u8 >) ;
};
}
