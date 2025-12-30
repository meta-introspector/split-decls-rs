// Generated macro for impl_15 (impl)
macro_rules! Depcrateimpl_15 {
() => {
// Module: crate
// Provides: {"impl_15"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl AsciiChars for String { # [inline (always)] fn ascii_chars (& self) -> AsciiCharsIter < '_ > { AsciiCharsIter :: new (self) } # [inline (always)] fn to_ascii_lossy (& self) -> String { deunicode (self) } }
};
}
