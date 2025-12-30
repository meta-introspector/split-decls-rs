// Generated macro for ParseError (enum)
macro_rules! DepcrateParseError {
() => {
// Module: crate
// Provides: {"ParseError"}
// Dependencies: {}
# [doc = " An error involving FixedDecimal operations or conversion."] # [derive (Display , Debug , Copy , Clone , PartialEq)] # [non_exhaustive] pub enum ParseError { # [doc = " See [`LimitError`]."] # [displaydoc ("Magnitude or number of digits exceeded")] Limit , # [doc = " The input of a string that is supposed to be converted to FixedDecimal is not accepted."] # [doc = ""] # [doc = " Any string with non-digit characters (except for one '.' and one '-' at the beginning of the string) is not accepted."] # [doc = " Also, empty string (\"\") and its negation (\"-\") are not accepted."] # [doc = " Strings of form \"12_345_678\" are not accepted, the accepted format is \"12345678\"."] # [doc = " Also '.' shouldn't be first or the last characters, i. e. .123 and 123. are not accepted, and instead 0.123 and"] # [doc = " 123 (or 123.0) must be used."] # [displaydoc ("Failed to parse the input string")] Syntax , }
};
}
