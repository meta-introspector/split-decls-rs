// Generated macro for EscapeError (enum)
macro_rules! Depcrate_escapeEscapeError {
() => {
// Module: crate::escape
// Provides: {"EscapeError"}
// Dependencies: {}
# [doc = " Error for XML escape / unescape."] # [derive (Clone , Debug , PartialEq)] pub enum EscapeError { # [doc = " Referenced entity in unknown to the parser."] UnrecognizedEntity (Range < usize > , String) , # [doc = " Cannot find `;` after `&`"] UnterminatedEntity (Range < usize >) , # [doc = " Attempt to parse character reference (`&#<dec-number>;` or `&#x<hex-number>;`)"] # [doc = " was unsuccessful, not all characters are decimal or hexadecimal numbers."] InvalidCharRef (ParseCharRefError) , }
};
}
