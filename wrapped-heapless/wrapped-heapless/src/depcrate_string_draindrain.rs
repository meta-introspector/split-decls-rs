// Generated macro for Drain (struct)
macro_rules! Depcrate_string_drainDrain {
() => {
// Module: crate::string::drain
// Provides: {"Drain"}
// Dependencies: {}
# [doc = " A draining iterator for `String`."] # [doc = ""] # [doc = " This struct is created by the [`drain`] method on [`crate::String`]. See its"] # [doc = " documentation for more."] # [doc = ""] # [doc = " [`drain`]: crate::String::drain"] pub struct Drain < 'a , LenT : LenType > { # [doc = " Will be used as &'a mut String in the destructor"] pub (super) string : * mut StringView < LenT > , # [doc = " Stast of part to remove"] pub (super) start : LenT , # [doc = " End of part to remove"] pub (super) end : LenT , # [doc = " Current remaining range to remove"] pub (super) iter : Chars < 'a > , }
};
}
