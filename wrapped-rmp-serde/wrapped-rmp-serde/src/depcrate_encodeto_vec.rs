// Generated macro for to_vec (function)
macro_rules! Depcrate_encodeto_vec {
() => {
// Module: crate::encode
// Provides: {"to_vec"}
// Dependencies: {}
# [doc = " Serialize the given data structure as a MessagePack byte vector."] # [doc = " This method uses compact representation, structs are serialized as arrays"] # [doc = ""] # [doc = " Serialization can fail if `T`'s implementation of `Serialize` decides to fail."] # [inline] pub fn to_vec < T > (val : & T) -> Result < Vec < u8 > , Error > where T : Serialize + ? Sized , { let mut wr = FallibleWriter (Vec :: new ()) ; write (& mut wr , val) ? ; Ok (wr . 0) }
};
}
