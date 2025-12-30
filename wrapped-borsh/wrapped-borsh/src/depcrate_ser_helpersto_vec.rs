// Generated macro for to_vec (function)
macro_rules! Depcrate_ser_helpersto_vec {
() => {
// Module: crate::ser::helpers
// Provides: {"to_vec"}
// Dependencies: {}
# [doc = " Serialize an object into a vector of bytes."] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " assert_eq!(vec![12, 0, 0, 0, 0, 0, 0, 0], borsh::to_vec(&12u64).unwrap());"] # [doc = " ```"] pub fn to_vec < T > (value : & T) -> Result < Vec < u8 > > where T : BorshSerialize + ? Sized , { let mut result = Vec :: with_capacity (DEFAULT_SERIALIZER_CAPACITY) ; value . serialize (& mut result) ? ; Ok (result) }
};
}
