// Generated macro for impl_96 (impl)
macro_rules! Depcrate_sourcesimpl_96 {
() => {
// Module: crate::sources
// Provides: {"impl_96"}
// Dependencies: {}
# [doc = " By default, a line diff is produced for a string"] impl < 'a > TokenSource for & 'a str { type Token = & 'a str ; type Tokenizer = Lines < 'a > ; fn tokenize (& self) -> Self :: Tokenizer { lines (self) } fn estimate_tokens (& self) -> u32 { lines (self) . estimate_tokens () } }
};
}
