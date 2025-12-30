// Generated macro for impl_16 (impl)
macro_rules! Depcrateimpl_16 {
() => {
// Module: crate
// Provides: {"impl_16"}
// Dependencies: {}
impl AsciiChars for str { # [inline (always)] fn ascii_chars (& self) -> AsciiCharsIter < '_ > { AsciiCharsIter :: new (self) } # [inline (always)] # [cfg (feature = "alloc")] fn to_ascii_lossy (& self) -> String { deunicode (self) } }
};
}
