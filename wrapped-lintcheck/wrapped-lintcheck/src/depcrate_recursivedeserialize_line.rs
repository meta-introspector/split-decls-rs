// Generated macro for deserialize_line (function)
macro_rules! Depcrate_recursivedeserialize_line {
() => {
// Module: crate::recursive
// Provides: {"deserialize_line"}
// Dependencies: {}
pub (crate) fn deserialize_line < T , R > (reader : & mut R) -> T where T : DeserializeOwned , R : BufRead , { let mut string = String :: new () ; reader . read_line (& mut string) . expect ("read_line failed") ; serde_json :: from_str (& string) . expect ("failed to deserialize") }
};
}
