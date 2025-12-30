// Generated macro for CharRange (enum)
macro_rules! Depcrate_manual_is_ascii_checkCharRange {
() => {
// Module: crate::manual_is_ascii_check
// Provides: {"CharRange"}
// Dependencies: {}
# [derive (Debug , PartialEq)] enum CharRange { # [doc = " 'a'..='z' | b'a'..=b'z'"] LowerChar , # [doc = " 'A'..='Z' | b'A'..=b'Z'"] UpperChar , # [doc = " `AsciiLower` | `AsciiUpper`"] FullChar , # [doc = " '0..=9'"] Digit , # [doc = " 'a..=f'"] LowerHexLetter , # [doc = " 'A..=F'"] UpperHexLetter , # [doc = " '0..=9' | 'a..=f' | 'A..=F'"] HexDigit , Otherwise , }
};
}
