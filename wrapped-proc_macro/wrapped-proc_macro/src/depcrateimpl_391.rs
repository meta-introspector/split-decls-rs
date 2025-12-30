// Generated macro for impl_391 (impl)
macro_rules! Depcrateimpl_391 {
() => {
// Module: crate
// Provides: {"impl_391"}
// Dependencies: {}
# [doc = " Parse a single literal from its stringified representation."] # [doc = ""] # [doc = " In order to parse successfully, the input string must not contain anything"] # [doc = " but the literal token. Specifically, it must not contain whitespace or"] # [doc = " comments in addition to the literal."] # [doc = ""] # [doc = " The resulting literal token will have a `Span::call_site()` span."] # [doc = ""] # [doc = " NOTE: some errors may cause panics instead of returning `LexError`. We"] # [doc = " reserve the right to change these errors into `LexError`s later."] # [stable (feature = "proc_macro_literal_parse" , since = "1.54.0")] impl FromStr for Literal { type Err = LexError ; fn from_str (src : & str) -> Result < Self , LexError > { match bridge :: client :: FreeFunctions :: literal_from_str (src) { Ok (literal) => Ok (Literal (literal)) , Err (()) => Err (LexError) , } } }
};
}
