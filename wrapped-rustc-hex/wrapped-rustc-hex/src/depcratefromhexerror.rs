// Generated macro for FromHexError (enum)
macro_rules! DepcrateFromHexError {
() => {
// Module: crate
// Provides: {"FromHexError"}
// Dependencies: {}
# [doc = " Errors that can occur when decoding a hex encoded string"] # [derive (Clone , Copy)] pub enum FromHexError { # [doc = " The input contained a character not part of the hex format"] InvalidHexCharacter (char , usize) , # [doc = " The input had an invalid length"] InvalidHexLength , }
};
}
