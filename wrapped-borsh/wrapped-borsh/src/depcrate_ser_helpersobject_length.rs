// Generated macro for object_length (function)
macro_rules! Depcrate_ser_helpersobject_length {
() => {
// Module: crate::ser::helpers
// Provides: {"object_length"}
// Dependencies: {}
# [doc = " Serializes an object without allocation to compute and return its length"] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use borsh::BorshSerialize;"] # [doc = ""] # [doc = " /// derive is only available if borsh is built with `features = [\"derive\"]`"] # [doc = " # #[cfg(feature = \"derive\")]"] # [doc = " #[derive(BorshSerialize)]"] # [doc = " struct A {"] # [doc = "     tag: String,"] # [doc = "     value: u64,"] # [doc = " };"] # [doc = ""] # [doc = " # #[cfg(feature = \"derive\")]"] # [doc = " let a = A { tag: \"hello\".to_owned(), value: 42 };"] # [doc = ""] # [doc = " assert_eq!(8, borsh::object_length(&12u64).unwrap());"] # [doc = " # #[cfg(feature = \"derive\")]"] # [doc = " assert_eq!(17, borsh::object_length(&a).unwrap());"] # [doc = " ```"] pub fn object_length < T > (value : & T) -> Result < usize > where T : BorshSerialize + ? Sized , { struct LengthWriter { len : usize , } impl Write for LengthWriter { # [inline] fn write (& mut self , buf : & [u8]) -> Result < usize > { let res = self . len . checked_add (buf . len ()) ; self . len = match res { Some (res) => res , None => { return Err (ErrorKind :: OutOfMemory . into ()) ; } } ; Ok (buf . len ()) } # [inline] fn flush (& mut self) -> Result < () > { Ok (()) } } let mut w = LengthWriter { len : 0 } ; value . serialize (& mut w) ? ; Ok (w . len) }
};
}
