// Generated macro for from_slice (function)
macro_rules! Depcrate_decodefrom_slice {
() => {
// Module: crate::decode
// Provides: {"from_slice"}
// Dependencies: {}
# [doc = " Deserialize a temporary scope-bound instance of type `T` from a slice, with zero-copy if possible."] # [doc = ""] # [doc = " Deserialization will be performed in zero-copy manner whenever it is possible, borrowing the"] # [doc = " data from the slice itself. For example, strings and byte-arrays won't copied."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " This conversion can fail if the structure of the Value does not match the structure expected"] # [doc = " by `T`. It can also fail if the structure is correct but `T`'s implementation of `Deserialize`"] # [doc = " decides that something is wrong with the data, for example required struct fields are missing."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use serde::Deserialize;"] # [doc = ""] # [doc = " // Encoded `[\"Bobby\", 8]`."] # [doc = " let buf = [0x92, 0xa5, 0x42, 0x6f, 0x62, 0x62, 0x79, 0x8];"] # [doc = ""] # [doc = " #[derive(Debug, Deserialize, PartialEq)]"] # [doc = " struct Dog<'a> {"] # [doc = "    name: &'a str,"] # [doc = "    age: u8,"] # [doc = " }"] # [doc = ""] # [doc = " assert_eq!(Dog { name: \"Bobby\", age: 8 }, rmp_serde::from_slice(&buf).unwrap());"] # [doc = " ```"] # [inline (always)] # [allow (deprecated)] pub fn from_slice < 'a , T > (input : & 'a [u8]) -> Result < T , Error > where T : Deserialize < 'a > , { from_read_ref (input) }
};
}
