// Generated macro for from_str (function)
macro_rules! Depcrate_defrom_str {
() => {
// Module: crate::de
// Provides: {"from_str"}
// Dependencies: {}
# [doc = " Attempts to deserialize the config from a string slice."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " Will return a `DeserializationError` if the config is invalid."] pub fn from_str < T > (s : & str) -> Result < T > where T : de :: DeserializeOwned , { let mut deserializer = Deserializer :: from_str (s) ? ; T :: deserialize (& mut deserializer) }
};
}
