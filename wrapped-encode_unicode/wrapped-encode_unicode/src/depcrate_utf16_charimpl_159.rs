// Generated macro for impl_159 (impl)
macro_rules! Depcrate_utf16_charimpl_159 {
() => {
// Module: crate::utf16_char
// Provides: {"impl_159"}
// Dependencies: {}
impl From < char > for Utf16Char { fn from (c : char) -> Self { let (first , second) = c . to_utf16_tuple () ; Utf16Char { units : [first , second . unwrap_or (0)] } } }
};
}
