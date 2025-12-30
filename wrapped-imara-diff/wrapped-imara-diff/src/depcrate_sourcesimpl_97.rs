// Generated macro for impl_97 (impl)
macro_rules! Depcrate_sourcesimpl_97 {
() => {
// Module: crate::sources
// Provides: {"impl_97"}
// Dependencies: {}
# [doc = " By default, a line diff is produced for a bytes"] impl < 'a > TokenSource for & 'a [u8] { type Token = Self ; type Tokenizer = ByteLines < 'a > ; fn tokenize (& self) -> Self :: Tokenizer { byte_lines (self) } fn estimate_tokens (& self) -> u32 { byte_lines (self) . estimate_tokens () } }
};
}
