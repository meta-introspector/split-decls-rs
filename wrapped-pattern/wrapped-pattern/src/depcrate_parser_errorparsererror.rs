// Generated macro for ParserError (enum)
macro_rules! Depcrate_parser_errorParserError {
() => {
// Module: crate::parser::error
// Provides: {"ParserError"}
// Dependencies: {}
# [doc = " An error returned when parsing a pattern."] # [doc = ""] # [doc = " ✨ *Enabled with the `alloc` Cargo feature.*"] # [doc = ""] # [doc = " # Examples"] # [doc = " ```"] # [doc = " use icu_pattern::{Parser, ParserError, ParserOptions};"] # [doc = ""] # [doc = " let mut parser = Parser::<usize>::new(\"{0\", ParserOptions::default());"] # [doc = " assert_eq!(Err(ParserError::UnclosedPlaceholder), parser.try_next());"] # [doc = " ```"] # [doc = ""] # [doc = " # Type parameters"] # [doc = ""] # [doc = " - `E`: An error of the replacement type which implements [`FromStr`]."] # [doc = ""] # [doc = " [`FromStr`]: core::str::FromStr"] # [derive (Display , Debug , PartialEq)] # [non_exhaustive] pub enum ParserError < E > where E : Debug , { # [doc = " Encountered an illegal character."] # [displaydoc ("Illegal character: {0}.")] IllegalCharacter (char) , # [doc = " Placeholder hould not be parsed from the given string slice."] # [displaydoc ("Invalid placeholder: {0:?}")] InvalidPlaceholder (E) , # [doc = " The pattern contains an unclosed placeholder."] # [displaydoc ("Unclosed placeholder")] UnclosedPlaceholder , # [doc = " The pattern contains an unclosed quoted literal."] # [displaydoc ("Unclosed quoted literal")] UnclosedQuotedLiteral , }
};
}
