// Generated macro for Dict (struct)
macro_rules! Depcrate_marshalledDict {
() => {
// Module: crate::marshalled
// Provides: {"Dict"}
// Dependencies: {}
# [doc = " Contains multiple keys and values, where every key is of the same type"] # [doc = " and every value is of the same type."] # [derive (Debug , Clone , Copy)] pub struct Dict < 'a > { outer_sig : & 'a SignatureSingle , key_sig : & 'a SignatureSingle , value_sig : & 'a SignatureSingle , data : & 'a [u8] , is_big_endian : bool , }
};
}
