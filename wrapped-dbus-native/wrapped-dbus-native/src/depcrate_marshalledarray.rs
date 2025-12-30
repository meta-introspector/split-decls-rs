// Generated macro for Array (struct)
macro_rules! Depcrate_marshalledArray {
() => {
// Module: crate::marshalled
// Provides: {"Array"}
// Dependencies: {}
# [doc = " Contains multiple values of the same type."] # [derive (Debug , Clone , Copy)] pub struct Array < 'a > { inner_sig : & 'a SignatureSingle , data : & 'a [u8] , start_pos : usize , is_big_endian : bool , }
};
}
