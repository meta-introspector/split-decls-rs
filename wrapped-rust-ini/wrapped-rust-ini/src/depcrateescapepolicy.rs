// Generated macro for EscapePolicy (enum)
macro_rules! DepcrateEscapePolicy {
() => {
// Module: crate
// Provides: {"EscapePolicy"}
// Dependencies: {}
# [doc = " Policies for escaping logic"] # [derive (Debug , PartialEq , Copy , Clone)] pub enum EscapePolicy { # [doc = " Escape absolutely nothing (dangerous)"] Nothing , # [doc = " Only escape the most necessary things."] # [doc = " This means backslashes, control characters (codepoints U+0000 to U+001F), and delete (U+007F)."] # [doc = " Quotes (single or double) are not escaped."] Basics , # [doc = " Escape basics and non-ASCII characters in the [Basic Multilingual Plane](https://www.compart.com/en/unicode/plane)"] # [doc = " (i.e. between U+007F - U+FFFF)"] # [doc = " Codepoints above U+FFFF, e.g. '🐱' U+1F431 \"CAT FACE\" will *not* be escaped!"] BasicsUnicode , # [doc = " Escape basics and all non-ASCII characters, including codepoints above U+FFFF."] # [doc = " This will escape emoji - if you want them to remain raw, use BasicsUnicode instead."] BasicsUnicodeExtended , # [doc = " Escape reserved symbols."] # [doc = " This includes everything in EscapePolicy::Basics, plus the comment characters ';' and '#' and the key/value-separating characters '=' and ':'."] Reserved , # [doc = " Escape reserved symbols and non-ASCII characters in the BMP."] # [doc = " Codepoints above U+FFFF, e.g. '🐱' U+1F431 \"CAT FACE\" will *not* be escaped!"] ReservedUnicode , # [doc = " Escape reserved symbols and all non-ASCII characters, including codepoints above U+FFFF."] ReservedUnicodeExtended , # [doc = " Escape everything that some INI implementations assume"] Everything , }
};
}
