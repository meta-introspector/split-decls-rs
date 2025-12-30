// Generated macro for Substrings (struct)
macro_rules! Depcrate_stringSubstrings {
() => {
// Module: crate::string
// Provides: {"Substrings"}
// Dependencies: {}
# [doc = " Iterator of all non-empty substrings"] # [derive (Clone)] pub struct Substrings < 'a > { iter : iter :: FlatMap < Prefixes < 'a > , Suffixes < 'a > , fn (& 'a str) -> Suffixes < 'a > > , }
};
}
