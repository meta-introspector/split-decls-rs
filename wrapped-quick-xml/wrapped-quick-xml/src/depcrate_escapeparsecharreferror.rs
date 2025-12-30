// Generated macro for ParseCharRefError (enum)
macro_rules! Depcrate_escapeParseCharRefError {
() => {
// Module: crate::escape
// Provides: {"ParseCharRefError"}
// Dependencies: {}
# [doc = " Error of parsing character reference (`&#<dec-number>;` or `&#x<hex-number>;`)."] # [derive (Clone , Debug , PartialEq)] pub enum ParseCharRefError { # [doc = " Number contains sign character (`+` or `-`) which is not allowed."] UnexpectedSign , # [doc = " Number cannot be parsed due to non-number characters or a numeric overflow."] InvalidNumber (ParseIntError) , # [doc = " Character reference represents not a valid unicode codepoint."] InvalidCodepoint (u32) , # [doc = " Character reference expanded to a not permitted character for an XML."] # [doc = ""] # [doc = " Currently, only `0x0` character produces this error."] IllegalCharacter (u32) , }
};
}
