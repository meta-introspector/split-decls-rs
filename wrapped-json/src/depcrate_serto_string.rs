// Generated macro for to_string (function)
macro_rules! Depcrate_serto_string {
() => {
// Module: crate::ser
// Provides: {"to_string"}
// Dependencies: {}
# [doc = " Serialize the given data structure as a String of JSON."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " Serialization can fail if `T`'s implementation of `Serialize` decides to"] # [doc = " fail, or if `T` contains a map with non-string keys."] # [inline] pub fn to_string < T > (value : & T) -> Result < String > where T : ? Sized + Serialize , { let vec = tri ! (to_vec (value)) ; let string = unsafe { String :: from_utf8_unchecked (vec) } ; Ok (string) }
};
}
