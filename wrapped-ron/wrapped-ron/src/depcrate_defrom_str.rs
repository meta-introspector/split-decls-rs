// Generated macro for from_str (function)
macro_rules! Depcrate_defrom_str {
() => {
// Module: crate::de
// Provides: {"from_str"}
// Dependencies: {}
# [doc = " A convenience function for building a deserializer"] # [doc = " and deserializing a value of type `T` from a string."] pub fn from_str < 'a , T > (s : & 'a str) -> SpannedResult < T > where T : de :: Deserialize < 'a > , { Options :: default () . from_str (s) }
};
}
