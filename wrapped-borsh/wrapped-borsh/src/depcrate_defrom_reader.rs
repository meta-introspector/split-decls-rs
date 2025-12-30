// Generated macro for from_reader (function)
macro_rules! Depcrate_defrom_reader {
() => {
// Module: crate::de
// Provides: {"from_reader"}
// Dependencies: {}
# [doc = " Deserializes an object from a reader."] # [doc = " # Example"] # [doc = " ```"] # [doc = " use borsh::{BorshDeserialize, BorshSerialize, from_reader, to_vec};"] # [doc = ""] # [doc = " /// derive is only available if borsh is built with `features = [\"derive\"]`"] # [doc = " # #[cfg(feature = \"derive\")]"] # [doc = " #[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug)]"] # [doc = " struct MyStruct {"] # [doc = "     a: u64,"] # [doc = "     b: Vec<u8>,"] # [doc = " }"] # [doc = ""] # [doc = " # #[cfg(feature = \"derive\")]"] # [doc = " let original = MyStruct { a: 10, b: vec![1, 2, 3] };"] # [doc = " # #[cfg(feature = \"derive\")]"] # [doc = " let encoded = to_vec(&original).unwrap();"] # [doc = " # #[cfg(feature = \"derive\")]"] # [doc = " let decoded = from_reader::<_, MyStruct>(&mut encoded.as_slice()).unwrap();"] # [doc = " # #[cfg(feature = \"derive\")]"] # [doc = " assert_eq!(original, decoded);"] # [doc = " ```"] pub fn from_reader < R : Read , T : BorshDeserialize > (reader : & mut R) -> Result < T > { let result = T :: deserialize_reader (reader) ? ; let mut buf = [0u8 ; 1] ; match reader . read_exact (& mut buf) { Err (f) if f . kind () == ErrorKind :: UnexpectedEof => Ok (result) , _ => Err (Error :: new (ErrorKind :: InvalidData , ERROR_NOT_ALL_BYTES_READ)) , } }
};
}
