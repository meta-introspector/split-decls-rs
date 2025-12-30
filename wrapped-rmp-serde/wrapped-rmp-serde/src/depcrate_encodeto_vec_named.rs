// Generated macro for to_vec_named (function)
macro_rules! Depcrate_encodeto_vec_named {
() => {
// Module: crate::encode
// Provides: {"to_vec_named"}
// Dependencies: {}
# [doc = " Serializes data structure into byte vector as a map"] # [doc = " Resulting MessagePack message will contain field names"] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " Serialization can fail if `T`'s implementation of `Serialize` decides to fail."] # [inline] pub fn to_vec_named < T > (val : & T) -> Result < Vec < u8 > , Error > where T : Serialize + ? Sized , { let mut wr = FallibleWriter (Vec :: new ()) ; write_named (& mut wr , val) ? ; Ok (wr . 0) }
};
}
