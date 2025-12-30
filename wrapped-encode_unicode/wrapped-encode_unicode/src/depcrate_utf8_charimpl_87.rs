// Generated macro for impl_87 (impl)
macro_rules! Depcrate_utf8_charimpl_87 {
() => {
// Module: crate::utf8_char
// Provides: {"impl_87"}
// Dependencies: {}
impl AsRef < str > for Utf8Char { fn as_ref (& self) -> & str { unsafe { str :: from_utf8_unchecked (self . as_ref ()) } } }
};
}
