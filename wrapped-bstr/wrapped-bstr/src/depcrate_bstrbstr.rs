// Generated macro for BStr (struct)
macro_rules! Depcrate_bstrBStr {
() => {
// Module: crate::bstr
// Provides: {"BStr"}
// Dependencies: {}
# [doc = " A wrapper for `&[u8]` that provides convenient string oriented trait impls."] # [doc = ""] # [doc = " If you need ownership or a growable byte string buffer, then use"] # [doc = " [`BString`](struct.BString.html)."] # [doc = ""] # [doc = " Using a `&BStr` is just like using a `&[u8]`, since `BStr`"] # [doc = " implements `Deref` to `[u8]`. So all methods available on `[u8]`"] # [doc = " are also available on `BStr`."] # [doc = ""] # [doc = " # Representation"] # [doc = ""] # [doc = " A `&BStr` has the same representation as a `&str`. That is, a `&BStr` is"] # [doc = " a fat pointer which consists of a pointer to some bytes and a length."] # [doc = ""] # [doc = " # Trait implementations"] # [doc = ""] # [doc = " The `BStr` type has a number of trait implementations, and in particular,"] # [doc = " defines equality and ordinal comparisons between `&BStr`, `&str` and"] # [doc = " `&[u8]` for convenience."] # [doc = ""] # [doc = " The `Debug` implementation for `BStr` shows its bytes as a normal string."] # [doc = " For invalid UTF-8, hex escape sequences are used."] # [doc = ""] # [doc = " The `Display` implementation behaves as if `BStr` were first lossily"] # [doc = " converted to a `str`. Invalid UTF-8 bytes are substituted with the Unicode"] # [doc = " replacement codepoint, which looks like this: �."] # [repr (transparent)] pub struct BStr { pub (crate) bytes : [u8] , }
};
}
