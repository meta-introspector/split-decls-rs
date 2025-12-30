// Generated macro for impl_21 (impl)
macro_rules! Depcrateimpl_21 {
() => {
// Module: crate
// Provides: {"impl_21"}
// Dependencies: {}
# [doc = " Format without a temporary string"] # [doc = ""] # [cfg_attr (feature = "alloc" , doc = "```rust")] # [cfg_attr (not (feature = "alloc") , doc = "```rust,ignore")] # [doc = " use deunicode::AsciiChars;"] # [doc = " format!(\"what's up {}\", \"🐶\".ascii_chars());"] # [doc = "```"] impl core :: fmt :: Display for AsciiCharsIter < '_ > { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { self . clone () . try_for_each (| ch | f . write_str (ch . unwrap_or ("\u{FFFD}"))) } }
};
}
