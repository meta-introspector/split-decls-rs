// Generated macro for FromHexError (enum)
macro_rules! Depcrate_errorFromHexError {
() => {
// Module: crate::error
// Provides: {"FromHexError"}
// Dependencies: {}
# [doc = " The error type for decoding a hex string into `Vec<u8>` or `[u8; N]`."] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub enum FromHexError { # [doc = " An invalid character was found. Valid ones are: `0...9`, `a...f`"] # [doc = " or `A...F`."] InvalidHexCharacter { c : char , index : usize } , # [doc = " A hex string's length needs to be even, as two digits correspond to"] # [doc = " one byte."] OddLength , # [doc = " If the hex string is decoded into a fixed sized container, such as an"] # [doc = " array, the hex string's length * 2 has to match the container's"] # [doc = " length."] InvalidStringLength , }
};
}
