// Generated macro for PosixParseError (enum)
macro_rules! Depcrate_locale_posixPosixParseError {
() => {
// Module: crate::locale::posix
// Provides: {"PosixParseError"}
// Dependencies: {}
# [derive (Display , Debug , PartialEq)] # [doc = " An error while parsing a POSIX locale identifier"] pub enum PosixParseError { # [displaydoc ("Empty locale")] EmptyLocale , # [displaydoc ("Empty section beginning at offset {offset}")] EmptySection { offset : usize } , # [displaydoc ("Invalid character at offset {offset}")] InvalidCharacter { offset : usize } , # [displaydoc ("Invalid locale")] InvalidLocale , # [displaydoc ("Delimiter repeated at offsets {first_offset} and {second_offset}")] RepeatedDelimiter { first_offset : usize , second_offset : usize , } , # [displaydoc ("Delimiters found out-of-order at offsets {first_offset} and {second_offset}")] UnorderedDelimiter { first_offset : usize , second_offset : usize , } , }
};
}
