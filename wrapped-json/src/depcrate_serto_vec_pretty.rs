// Generated macro for to_vec_pretty (function)
macro_rules! Depcrate_serto_vec_pretty {
() => {
// Module: crate::ser
// Provides: {"to_vec_pretty"}
// Dependencies: {}
# [doc = " Serialize the given data structure as a pretty-printed JSON byte vector."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " Serialization can fail if `T`'s implementation of `Serialize` decides to"] # [doc = " fail, or if `T` contains a map with non-string keys."] # [inline] pub fn to_vec_pretty < T > (value : & T) -> Result < Vec < u8 > > where T : ? Sized + Serialize , { let mut writer = Vec :: with_capacity (128) ; tri ! (to_writer_pretty (& mut writer , value)) ; Ok (writer) }
};
}
