// Generated macro for from_bytes (function)
macro_rules! Depcrate_defrom_bytes {
() => {
// Module: crate::de
// Provides: {"from_bytes"}
// Dependencies: {}
# [doc = " A convenience function for building a deserializer"] # [doc = " and deserializing a value of type `T` from bytes."] pub fn from_bytes < 'a , T > (s : & 'a [u8]) -> SpannedResult < T > where T : de :: Deserialize < 'a > , { Options :: default () . from_bytes (s) }
};
}
