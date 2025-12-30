// Generated macro for to_string_pretty (function)
macro_rules! Depcrate_serto_string_pretty {
() => {
// Module: crate::ser
// Provides: {"to_string_pretty"}
// Dependencies: {}
# [doc = " Serialize the given data structure as a pretty-printed String of JSON."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " Serialization can fail if `T`'s implementation of `Serialize` decides to"] # [doc = " fail, or if `T` contains a map with non-string keys."] # [inline] pub fn to_string_pretty < T > (value : & T) -> Result < String > where T : ? Sized + Serialize , { let vec = tri ! (to_vec_pretty (value)) ; let string = unsafe { String :: from_utf8_unchecked (vec) } ; Ok (string) }
};
}
