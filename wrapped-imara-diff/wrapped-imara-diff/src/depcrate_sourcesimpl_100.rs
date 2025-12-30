// Generated macro for impl_100 (impl)
macro_rules! Depcrate_sourcesimpl_100 {
() => {
// Module: crate::sources
// Provides: {"impl_100"}
// Dependencies: {}
# [doc = " By default a line diff is produced for a string"] impl < 'a > TokenSource for Lines < 'a > { type Token = & 'a str ; type Tokenizer = Self ; fn tokenize (& self) -> Self :: Tokenizer { * self } fn estimate_tokens (& self) -> u32 { self . 0 . estimate_tokens () } }
};
}
