// Generated macro for from_slice (function)
macro_rules! Depcrate_defrom_slice {
() => {
// Module: crate::de
// Provides: {"from_slice"}
// Dependencies: {}
# [doc = " Deserializes an object from a slice of bytes."] # [doc = " # Example"] # [doc = " ```"] # [doc = " use borsh::{BorshDeserialize, BorshSerialize, from_slice, to_vec};"] # [doc = ""] # [doc = " /// derive is only available if borsh is built with `features = [\"derive\"]`"] # [doc = " # #[cfg(feature = \"derive\")]"] # [doc = " #[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug)]"] # [doc = " struct MyStruct {"] # [doc = "    a: u64,"] # [doc = "    b: Vec<u8>,"] # [doc = " }"] # [doc = ""] # [doc = " # #[cfg(feature = \"derive\")]"] # [doc = " let original = MyStruct { a: 10, b: vec![1, 2, 3] };"] # [doc = " # #[cfg(feature = \"derive\")]"] # [doc = " let encoded = to_vec(&original).unwrap();"] # [doc = " # #[cfg(feature = \"derive\")]"] # [doc = " let decoded = from_slice::<MyStruct>(&encoded).unwrap();"] # [doc = " # #[cfg(feature = \"derive\")]"] # [doc = " assert_eq!(original, decoded);"] # [doc = " ```"] # [doc = " # Panics"] # [doc = " If the data is invalid, this function will panic."] # [doc = " # Errors"] # [doc = " If the data is invalid, this function will return an error."] # [doc = " # Note"] # [doc = " This function will return an error if the data is not fully read."] pub fn from_slice < T : BorshDeserialize > (v : & [u8]) -> Result < T > { let mut v_mut = v ; let object = T :: deserialize (& mut v_mut) ? ; if ! v_mut . is_empty () { return Err (Error :: new (ErrorKind :: InvalidData , crate :: de :: ERROR_NOT_ALL_BYTES_READ ,)) ; } Ok (object) }
};
}
