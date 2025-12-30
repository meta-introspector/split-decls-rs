// Generated macro for StyledStr (struct)
macro_rules! Depcrate_builder_styled_strStyledStr {
() => {
// Module: crate::builder::styled_str
// Provides: {"StyledStr"}
// Dependencies: {}
# [doc = " Terminal-styling container"] # [doc = ""] # [doc = " Styling may be encoded as [ANSI Escape Code](https://en.wikipedia.org/wiki/ANSI_escape_code)"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " # use clap_builder as clap;"] # [doc = " // `cstr!` converts tags to ANSI codes"] # [doc = " let after_help: &'static str = color_print::cstr!("] # [doc = " r#\"<bold><underline>Examples</underline></bold>"] # [doc = ""] # [doc = "   <dim>$</dim> <bold>mybin --input file.toml</bold>"] # [doc = " \"#);"] # [doc = ""] # [doc = " let cmd = clap::Command::new(\"mybin\")"] # [doc = "     .after_help(after_help)  // The `&str` gets converted into a `StyledStr`"] # [doc = "     // ..."] # [doc = " #   ;"] # [doc = " ```"] # [derive (Clone , Default , Debug , PartialEq , Eq , PartialOrd , Ord)] pub struct StyledStr (String) ;
};
}
