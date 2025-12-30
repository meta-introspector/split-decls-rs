// Generated macro for from_reader (function)
macro_rules! Depcrate_defrom_reader {
() => {
// Module: crate::de
// Provides: {"from_reader"}
// Dependencies: {}
# [doc = " A convenience function for building a deserializer"] # [doc = " and deserializing a value of type `T` from a reader."] # [cfg (feature = "std")] pub fn from_reader < R , T > (rdr : R) -> SpannedResult < T > where R : io :: Read , T : de :: DeserializeOwned , { Options :: default () . from_reader (rdr) }
};
}
