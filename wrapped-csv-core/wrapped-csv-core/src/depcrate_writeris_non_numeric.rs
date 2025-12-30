// Generated macro for is_non_numeric (function)
macro_rules! Depcrate_writeris_non_numeric {
() => {
// Module: crate::writer
// Provides: {"is_non_numeric"}
// Dependencies: {}
# [doc = " Returns true if and only if the given input is non-numeric."] pub fn is_non_numeric (input : & [u8]) -> bool { let s = match str :: from_utf8 (input) { Err (_) => return true , Ok (s) => s , } ; s . parse :: < f64 > () . is_err () && s . parse :: < i128 > () . is_err () }
};
}
