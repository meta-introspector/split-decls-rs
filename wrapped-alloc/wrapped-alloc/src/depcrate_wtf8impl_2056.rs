// Generated macro for impl_2056 (impl)
macro_rules! Depcrate_wtf8impl_2056 {
() => {
// Module: crate::wtf8
// Provides: {"impl_2056"}
// Dependencies: {}
# [doc = " Formats the string in double quotes, with characters escaped according to"] # [doc = " [`char::escape_debug`] and unpaired surrogates represented as `\\u{xxxx}`,"] # [doc = " where each `x` is a hexadecimal digit."] # [doc = ""] # [doc = " For example, the code units [U+0061, U+D800, U+000A] are formatted as"] # [doc = " `\"a\\u{D800}\\n\"`."] impl fmt :: Debug for Wtf8Buf { # [inline] fn fmt (& self , formatter : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Debug :: fmt (& * * self , formatter) } }
};
}
