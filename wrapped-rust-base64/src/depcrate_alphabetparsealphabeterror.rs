// Generated macro for ParseAlphabetError (enum)
macro_rules! Depcrate_alphabetParseAlphabetError {
() => {
// Module: crate::alphabet
// Provides: {"ParseAlphabetError"}
// Dependencies: {}
# [doc = " Possible errors when constructing an [Alphabet] from a `str`."] # [derive (Debug , Eq , PartialEq)] pub enum ParseAlphabetError { # [doc = " Alphabets must be 64 ASCII bytes"] InvalidLength , # [doc = " All bytes must be unique"] DuplicatedByte (u8) , # [doc = " All bytes must be printable (in the range `[32, 126]`)."] UnprintableByte (u8) , # [doc = " `=` cannot be used"] ReservedByte (u8) , }
};
}
