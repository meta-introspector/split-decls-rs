// Generated macro for impl_315 (impl)
macro_rules! Depcrateimpl_315 {
() => {
// Module: crate
// Provides: {"impl_315"}
// Dependencies: {}
# [doc = " Attempts to break the string into tokens and parse those tokens into a token stream."] # [doc = " May fail for a number of reasons, for example, if the string contains unbalanced delimiters"] # [doc = " or characters not existing in the language."] # [doc = " All tokens in the parsed stream get `Span::call_site()` spans."] # [doc = ""] # [doc = " NOTE: some errors may cause panics instead of returning `LexError`. We reserve the right to"] # [doc = " change these errors into `LexError`s later."] # [stable (feature = "proc_macro_lib" , since = "1.15.0")] impl FromStr for TokenStream { type Err = LexError ; fn from_str (src : & str) -> Result < TokenStream , LexError > { Ok (TokenStream (Some (bridge :: client :: TokenStream :: from_str (src)))) } }
};
}
