// Generated macro for from_slice (function)
macro_rules! Depcrate_defrom_slice {
() => {
// Module: crate::de
// Provides: {"from_slice"}
// Dependencies: {}
# [doc = " Attempts to deserialize the config from a byte slice."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " Will return a `DeserializationError` if the config is invalid."] pub fn from_slice < T > (bytes : & [u8]) -> Result < T > where T : de :: DeserializeOwned , { match std :: str :: from_utf8 (bytes) { Ok (s) => from_str (s) , Err (e) => Err (Error :: DeserializationError (e . to_string ())) , } }
};
}
