// Generated macro for PatternError (enum)
macro_rules! Depcrate_provider_pattern_errorPatternError {
() => {
// Module: crate::provider::pattern::error
// Provides: {"PatternError"}
// Dependencies: {}
# [doc = " A low-level pattern parsing error."] # [doc = ""] # [doc = " These strings follow the recommendations for the serde::de::Unexpected::Other type."] # [doc = " <https://docs.serde.rs/serde/de/enum.Unexpected.html#variant.Other>"] # [doc = ""] # [doc = " Serde will generate an error such as:"] # [doc = " \"invalid value: unclosed literal in pattern, expected a valid UTS 35 pattern string at line 1 column 12\""] # [derive (Display , Debug , PartialEq , Copy , Clone)] # [allow (missing_docs)] # [non_exhaustive] pub enum PatternError { # [displaydoc ("{0:?} invalid field length in pattern")] FieldLengthInvalid (fields :: FieldSymbol) , # [displaydoc ("unknown substitution {0} in pattern")] UnknownSubstitution (char) , # [displaydoc ("invalid symbol {0} in pattern")] InvalidSymbol (char) , # [displaydoc ("unclosed literal in pattern")] UnclosedLiteral , # [displaydoc ("unclosed placeholder in pattern")] UnclosedPlaceholder , # [displaydoc ("plural pattern variants are only supported for week-of-month and week-of-year")] UnsupportedPluralPivot , }
};
}
