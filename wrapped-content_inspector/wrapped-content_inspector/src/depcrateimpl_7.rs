// Generated macro for impl_7 (impl)
macro_rules! Depcrateimpl_7 {
() => {
// Module: crate
// Provides: {"impl_7"}
// Dependencies: {}
impl ContentType { # [doc = " Returns `true`, if the `ContentType` is `BINARY`."] pub fn is_binary (self) -> bool { self == ContentType :: BINARY } # [doc = " Returns `true`, if the `ContentType` is __not__ `BINARY`."] pub fn is_text (self) -> bool { ! self . is_binary () } }
};
}
